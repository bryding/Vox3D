use std::collections::HashMap;

use wgpu::{ self, util::DeviceExt };
use wgpu_glyph::{ ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text };
use winit::window::Window;
use crate::game::GameState;

use super::fps_display::{ self, FpsDisplay };
use super::chunk_mesh::ChunkMesh;
use super::mesh_generator::MeshGenerator;
use super::{ camera::OPENGL_TO_WGPU_MATRIX, vertex::Vertex };
use super::camera::Camera;
use cgmath::{ perspective, Deg, Matrix4, Point3, SquareMatrix, Vector3 };

// This is the uniform buffer that will hold our camera matrix
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
    camera_pos: [f32; 3],
    _padding: u32, // Necessary for alignment
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
            camera_pos: [0.0, 0.0, 0.0],
            _padding: 0,
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
        self.camera_pos = camera.position.into();
    }
}

pub struct RenderState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    depth_texture: wgpu::TextureView,
    chunk_meshes: HashMap<(i32, i32), ChunkMesh>,
    fps_display: FpsDisplay,
    glyph_brush: GlyphBrush<()>,
    staging_belt: wgpu::util::StagingBelt,
}

impl RenderState {
    pub async fn new(window: &Window, game_state: &GameState) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = (unsafe { instance.create_surface(&window) }).unwrap();

        let adapter = instance
            .request_adapter(
                &(wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
            ).await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &(wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                }),
                None
            ).await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        // New camera setup
        let camera = Camera::new(size.width, size.height);

        // Create camera uniform and buffer
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(
            &(wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
        );

        // Create bind group layout
        let camera_bind_group_layout = device.create_bind_group_layout(
            &(wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT, // Note: added FRAGMENT
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
                label: Some("camera_bind_group_layout"),
            })
        );

        // Create bind group
        let camera_bind_group = device.create_bind_group(
            &(wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding(),
                    },
                ],
                label: Some("camera_bind_group"),
            })
        );

        // Create pipeline layout
        let render_pipeline_layout = device.create_pipeline_layout(
            &(wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            })
        );

        // Create shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
        });

        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(
            &(wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                    ],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            })
        );

        // Create depth texture
        let depth_texture = Self::create_depth_texture(&device, &config);

        let mut all_vertices = Vec::new();
        for chunk in game_state.chunks().values() {
            all_vertices.extend(
                MeshGenerator::generate_chunk_mesh(&chunk.voxels, chunk.chunk_x, chunk.chunk_z)
            );
        }
        let font = ab_glyph::FontArc
            ::try_from_slice(include_bytes!("../../assets/FiraSans-Regular.ttf"))
            .unwrap();
        let glyph_brush = GlyphBrushBuilder::using_font(font).build(&device, config.format);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            depth_texture,
            chunk_meshes: HashMap::new(),
            fps_display: FpsDisplay::new(),
            glyph_brush,
            staging_belt: wgpu::util::StagingBelt::new(1024),
        }
    }

    pub fn update(&mut self, game_state: &GameState) {
        // Update camera (same as before)
        self.update_camera(
            game_state.camera_position(),
            game_state.camera_direction(),
            game_state.camera_up()
        );

        self.camera_uniform.camera_pos = [
            game_state.camera_position().x,
            game_state.camera_position().y,
            game_state.camera_position().z,
        ];

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform])
        );

        // Only update chunks that were modified
        if game_state.chunks_updated() {
            for (pos, chunk) in game_state.chunks() {
                if !self.chunk_meshes.contains_key(pos) {
                    // Generate mesh for new chunk
                    let vertices = MeshGenerator::generate_chunk_mesh(
                        &chunk.voxels,
                        chunk.chunk_x,
                        chunk.chunk_z
                    );

                    if !vertices.is_empty() {
                        let chunk_mesh = ChunkMesh::new(&self.device, &vertices);
                        self.chunk_meshes.insert(*pos, chunk_mesh);
                    }
                }
            }

            // Remove meshes for unloaded chunks
            self.chunk_meshes.retain(|pos, _| game_state.chunks().contains_key(pos));
        }

        // Update FPS display
        self.fps_display.update();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            // Recreate depth texture with new size
            self.depth_texture = Self::create_depth_texture(&self.device, &self.config);

            // Update camera aspect ratio
            self.camera.resize(new_size.width, new_size.height);
            self.camera_uniform.update_view_proj(&self.camera);
            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera_uniform])
            );
        }
    }

    pub fn update_camera(
        &mut self,
        position: Point3<f32>,
        direction: Vector3<f32>,
        up: Vector3<f32>
    ) {
        // Update camera uniform with new camera data
        let view = Matrix4::look_to_rh(position, direction, up);
        let proj = perspective(
            Deg(45.0),
            (self.size.width as f32) / (self.size.height as f32),
            0.1,
            100.0
        );
        self.camera_uniform.view_proj = (OPENGL_TO_WGPU_MATRIX * proj * view).into();

        // Update GPU buffer
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform])
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &(wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            })
        );

        {
            let mut render_pass = encoder.begin_render_pass(
                &(wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        }),
                    ],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                })
            );

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            // Render each chunk separately
            for chunk_mesh in self.chunk_meshes.values() {
                render_pass.set_vertex_buffer(0, chunk_mesh.vertex_buffer.slice(..));
                render_pass.draw(0..chunk_mesh.num_vertices, 0..1);
            }
        }

        self.render_fps_display(&mut encoder, &view);
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn render_fps_display(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let scale = ((self.size.height as f32) / 40.0).round();

        self.glyph_brush.queue(Section {
            screen_position: ((self.size.width as f32) - 10.0, 10.0),
            bounds: (self.size.width as f32, self.size.height as f32),
            text: vec![
                Text::new(&format!("FPS: {}", self.fps_display.fps()))
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(scale)
            ],
            layout: wgpu_glyph::Layout::default().h_align(wgpu_glyph::HorizontalAlign::Right),
            ..Section::default()
        });

        self.glyph_brush
            .draw_queued(
                &self.device,
                &mut self.staging_belt,
                encoder,
                view,
                self.size.width,
                self.size.height
            )
            .expect("Failed to draw queued text");

        self.staging_belt.finish();
    }

    fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration
    ) -> wgpu::TextureView {
        let depth_texture = device.create_texture(
            &(wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width: config.width,
                    height: config.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                label: Some("depth_texture"),
                view_formats: &[],
            })
        );

        depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }
}
