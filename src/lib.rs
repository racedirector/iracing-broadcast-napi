#![deny(clippy::all)]

use iracing_broadcast::{
  BroadcastError, BroadcastMessage, ChatCommandMode as CrateChatCommandMode,
  Client as BroadcastClientImpl, PitCommandMode as CratePitCommandMode,
  ReplayPositionMode as CrateReplayPositionMode, ReplaySearchMode as CrateReplaySearchMode,
  TelemetryCommandMode as CrateTelemetryCommandMode, VideoCaptureMode as CrateVideoCaptureMode,
};
use napi::{Error, Result, Status};
use napi_derive::napi;

fn to_napi_error(err: BroadcastError) -> Error {
  Error::new(Status::GenericFailure, err.to_string())
}

#[napi]
pub enum ReplayPositionMode {
  Begin,
  Current,
  End,
}

impl From<ReplayPositionMode> for CrateReplayPositionMode {
  fn from(value: ReplayPositionMode) -> Self {
    match value {
      ReplayPositionMode::Begin => CrateReplayPositionMode::Begin,
      ReplayPositionMode::Current => CrateReplayPositionMode::Current,
      ReplayPositionMode::End => CrateReplayPositionMode::End,
    }
  }
}

#[napi]
pub enum ReplaySearchMode {
  ToStart,
  ToEnd,
  PreviousSession,
  NextSession,
  PreviousLap,
  NextLap,
  PreviousFrame,
  NextFrame,
  PreviousIncident,
  NextIncident,
}

impl From<ReplaySearchMode> for CrateReplaySearchMode {
  fn from(value: ReplaySearchMode) -> Self {
    match value {
      ReplaySearchMode::ToStart => CrateReplaySearchMode::ToStart,
      ReplaySearchMode::ToEnd => CrateReplaySearchMode::ToEnd,
      ReplaySearchMode::PreviousSession => CrateReplaySearchMode::PreviousSession,
      ReplaySearchMode::NextSession => CrateReplaySearchMode::NextSession,
      ReplaySearchMode::PreviousLap => CrateReplaySearchMode::PreviousLap,
      ReplaySearchMode::NextLap => CrateReplaySearchMode::NextLap,
      ReplaySearchMode::PreviousFrame => CrateReplaySearchMode::PreviousFrame,
      ReplaySearchMode::NextFrame => CrateReplaySearchMode::NextFrame,
      ReplaySearchMode::PreviousIncident => CrateReplaySearchMode::PreviousIncident,
      ReplaySearchMode::NextIncident => CrateReplaySearchMode::NextIncident,
    }
  }
}

#[napi]
pub enum TelemetryCommandMode {
  Stop,
  Start,
  Restart,
}

impl From<TelemetryCommandMode> for CrateTelemetryCommandMode {
  fn from(value: TelemetryCommandMode) -> Self {
    match value {
      TelemetryCommandMode::Stop => CrateTelemetryCommandMode::Stop,
      TelemetryCommandMode::Start => CrateTelemetryCommandMode::Start,
      TelemetryCommandMode::Restart => CrateTelemetryCommandMode::Restart,
    }
  }
}

#[napi]
pub enum ChatCommandMode {
  Macro,
  Begin,
  Reply,
  Cancel,
}

impl From<ChatCommandMode> for CrateChatCommandMode {
  fn from(value: ChatCommandMode) -> Self {
    match value {
      ChatCommandMode::Macro => CrateChatCommandMode::Macro,
      ChatCommandMode::Begin => CrateChatCommandMode::Begin,
      ChatCommandMode::Reply => CrateChatCommandMode::Reply,
      ChatCommandMode::Cancel => CrateChatCommandMode::Cancel,
    }
  }
}

#[napi]
pub enum VideoCaptureMode {
  ScreenShot,
  StartCapture,
  EndCapture,
  ToggleCapture,
  ShowTimer,
  HideTimer,
}

impl From<VideoCaptureMode> for CrateVideoCaptureMode {
  fn from(value: VideoCaptureMode) -> Self {
    match value {
      VideoCaptureMode::ScreenShot => CrateVideoCaptureMode::ScreenShot,
      VideoCaptureMode::StartCapture => CrateVideoCaptureMode::StartCapture,
      VideoCaptureMode::EndCapture => CrateVideoCaptureMode::EndCapture,
      VideoCaptureMode::ToggleCapture => CrateVideoCaptureMode::ToggleCapture,
      VideoCaptureMode::ShowTimer => CrateVideoCaptureMode::ShowTimer,
      VideoCaptureMode::HideTimer => CrateVideoCaptureMode::HideTimer,
    }
  }
}

#[napi]
pub enum PitCommandMode {
  Clear,
  Tearoff,
  Fuel,
  Lf,
  Rf,
  Lr,
  Rr,
  ClearTires,
  FastRepair,
  ClearTearoff,
  ClearFastRepair,
  ClearFuel,
}

impl PitCommandMode {
  fn into_crate(self, value: Option<u8>) -> std::result::Result<CratePitCommandMode, Error> {
    let missing_value = || Error::new(Status::InvalidArg, "pit command requires a value");

    let mode = match self {
      PitCommandMode::Clear => CratePitCommandMode::Clear,
      PitCommandMode::Tearoff => CratePitCommandMode::Tearoff,
      PitCommandMode::Fuel => CratePitCommandMode::Fuel(value.ok_or_else(missing_value)?),
      PitCommandMode::Lf => CratePitCommandMode::LF(value.ok_or_else(missing_value)?),
      PitCommandMode::Rf => CratePitCommandMode::RF(value.ok_or_else(missing_value)?),
      PitCommandMode::Lr => CratePitCommandMode::LR(value.ok_or_else(missing_value)?),
      PitCommandMode::Rr => CratePitCommandMode::RR(value.ok_or_else(missing_value)?),
      PitCommandMode::ClearTires => CratePitCommandMode::ClearTires,
      PitCommandMode::FastRepair => CratePitCommandMode::FastRepair,
      PitCommandMode::ClearTearoff => CratePitCommandMode::ClearTearoff,
      PitCommandMode::ClearFastRepair => CratePitCommandMode::ClearFastRepair,
      PitCommandMode::ClearFuel => CratePitCommandMode::ClearFuel,
    };

    Ok(mode)
  }
}

#[napi]
pub struct BroadcastClient {
  inner: BroadcastClientImpl,
}

#[napi]
impl BroadcastClient {
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    let inner = BroadcastClientImpl::new().map_err(to_napi_error)?;
    Ok(Self { inner })
  }

  #[napi]
  pub fn send_camera_switch_position(&self, position: u8, group: u8, camera: u8) -> Result<()> {
    self
      .inner
      .send_message(BroadcastMessage::CameraSwitchPosition(
        position, group, camera,
      ))
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn send_camera_switch_number(&self, car_number: String, group: u8, camera: u8) -> Result<()> {
    let leaked_number = Box::leak(car_number.into_boxed_str());
    self
      .inner
      .send_message(BroadcastMessage::CameraSwitchNumber(
        leaked_number,
        group,
        camera,
      ))
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn set_camera_state(&self, camera_state: u32) -> Result<()> {
    let state = iracing_broadcast::CameraState::from_bits_truncate(camera_state);
    self
      .inner
      .send_message(BroadcastMessage::CameraSetState(state))
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn set_replay_play_speed(&self, speed: u8, slow_motion: bool) -> Result<()> {
    self
      .inner
      .send_message(BroadcastMessage::ReplaySetPlaySpeed(speed, slow_motion))
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn set_replay_play_position(
    &self,
    mode: ReplayPositionMode,
    frame_number: u16,
  ) -> Result<()> {
    self
      .inner
      .send_message(BroadcastMessage::ReplaySetPlayPosition(
        mode.into(),
        frame_number,
      ))
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn search_replay(&self, mode: ReplaySearchMode) -> Result<()> {
    self
      .inner
      .send_message(BroadcastMessage::ReplaySearch(mode.into()))
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn toggle_replay_state(&self) -> Result<()> {
    self
      .inner
      .send_message(BroadcastMessage::ReplaySetState)
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn reload_all_textures(&self) -> Result<()> {
    self
      .inner
      .send_message(BroadcastMessage::ReloadAllTextures)
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn reload_textures(&self, car_index: u8) -> Result<()> {
    self
      .inner
      .send_message(BroadcastMessage::ReloadTextures(car_index))
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn send_chat_command(&self, mode: ChatCommandMode) -> Result<()> {
    self
      .inner
      .send_message(BroadcastMessage::ChatCommand(mode.into()))
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn send_chat_macro(&self, macro_number: u8) -> Result<()> {
    self
      .inner
      .send_message(BroadcastMessage::ChatCommandMacro(macro_number))
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn send_pit_command(&self, mode: PitCommandMode, value: Option<u8>) -> Result<()> {
    let command_mode = mode.into_crate(value)?;
    self
      .inner
      .send_message(BroadcastMessage::PitCommand(command_mode))
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn send_telemetry_command(&self, mode: TelemetryCommandMode) -> Result<()> {
    self
      .inner
      .send_message(BroadcastMessage::TelemetryCommand(mode.into()))
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn send_force_feedback_command(&self, value: u16) -> Result<()> {
    self
      .inner
      .send_message(BroadcastMessage::FFBCommand(value))
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn search_replay_session_time(&self, session_number: u8, session_time_ms: u16) -> Result<()> {
    self
      .inner
      .send_message(BroadcastMessage::ReplaySearchSessionTime(
        session_number,
        session_time_ms,
      ))
      .map_err(to_napi_error)
  }

  #[napi]
  pub fn control_video_capture(&self, mode: VideoCaptureMode) -> Result<()> {
    self
      .inner
      .send_message(BroadcastMessage::VideoCapture(mode.into()))
      .map_err(to_napi_error)
  }
}
