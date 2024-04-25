use libretro_rs::prelude::*;

pub const FRAMEBUFFER_WIDTH: u16 = 2;
pub const FRAMEBUFFER_HEIGHT: u16 = 2;
pub const FRAMEBUFFER_SIZE: usize = FRAMEBUFFER_WIDTH as usize * FRAMEBUFFER_HEIGHT as usize;

struct Core {
  rendering_mode: SoftwareRenderEnabled,
  format: ActiveFormat<XRGB8888>,
  frame_buffer: ArrayFrameBuffer<XRGB8888, FRAMEBUFFER_SIZE, FRAMEBUFFER_WIDTH>,
}

impl<'a> retro::Core<'a> for Core {
  type Init = ();

  fn get_system_info() -> SystemInfo {
    SystemInfo::new(
      c_utf8!("libretro-rs software rendering example"),
      c_utf8!(env!("CARGO_PKG_VERSION")),
      ext![],
    )
  }

  fn set_environment(env: &mut impl env::SetEnvironment) {
    env.set_support_no_game(true).ok();
  }

  fn init(_env: &mut impl env::Init) -> Self::Init {
    ()
  }

  fn load_without_content<E: env::LoadGame>(
    args: LoadGameExtraArgs<'a, '_, E, Self::Init>,
  ) -> Result<Self, CoreError> {
    let LoadGameExtraArgs { env, rendering_mode, pixel_format, .. } = args;
    let mut logger = FallbackLogger::new(env.get_log_interface().ok());
    logger.info(c_utf8!("Loading game."));
    let format = env.set_pixel_format_xrgb8888(pixel_format)?;
    logger.info(c_utf8!("Game loaded."));
    let framebuffer = ArrayFrameBuffer::new([
      XRGB8888::new_with_raw_value(0x000000FF),
      XRGB8888::new_with_raw_value(0x0000FF00),
      XRGB8888::new_with_raw_value(0x00FF0000),
      XRGB8888::new_with_raw_value(0x00FFFFFF),
    ]);
    Ok(Core { rendering_mode, format, frame_buffer: framebuffer })
  }

  fn get_system_av_info(&self, _env: &mut impl env::GetAvInfo) -> SystemAVInfo {
    SystemAVInfo::default_timings(GameGeometry::fixed(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT))
  }

  fn run(&mut self, _env: &mut impl env::Run, callbacks: &mut impl Callbacks) -> InputsPolled {
    callbacks.upload_video_frame(&self.rendering_mode, &self.format, &self.frame_buffer);
    callbacks.poll_inputs()
  }

  fn reset(&mut self, _env: &mut impl env::Reset) {}

  fn unload_game(self, _env: &mut impl env::UnloadGame) -> Self::Init {
    ()
  }
}

libretro_core!(crate::Core);
