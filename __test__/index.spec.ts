import test from 'ava'

import {
  BroadcastClient,
  ChatCommandMode,
  PitCommandMode,
  ReplayPositionMode,
  ReplaySearchMode,
  TelemetryCommandMode,
  VideoCaptureMode,
} from '../index'

test('exports broadcast bindings', (t) => {
  t.truthy(BroadcastClient)
  t.truthy(ReplayPositionMode)
  t.truthy(ReplaySearchMode)
  t.truthy(TelemetryCommandMode)
  t.truthy(ChatCommandMode)
  t.truthy(VideoCaptureMode)
  t.truthy(PitCommandMode)
})

test('client exposes message methods', (t) => {
  const prototype = BroadcastClient.prototype as Record<string, unknown>

  t.truthy(prototype.sendCameraSwitchPosition)
  t.truthy(prototype.sendCameraSwitchNumber)
  t.truthy(prototype.setCameraState)
  t.truthy(prototype.setReplayPlaySpeed)
  t.truthy(prototype.setReplayPlayPosition)
  t.truthy(prototype.searchReplay)
  t.truthy(prototype.toggleReplayState)
  t.truthy(prototype.reloadAllTextures)
  t.truthy(prototype.reloadTextures)
  t.truthy(prototype.sendChatCommand)
  t.truthy(prototype.sendChatMacro)
  t.truthy(prototype.sendPitCommand)
  t.truthy(prototype.sendTelemetryCommand)
  t.truthy(prototype.sendForceFeedbackCommand)
  t.truthy(prototype.searchReplaySessionTime)
  t.truthy(prototype.controlVideoCapture)
})
