CREATE TABLE "playback_history_events" (
    "id"          INTEGER PRIMARY KEY,
    "play_id"     INTEGER                           NOT NULL,
    "elapsed"     REAL                              NOT NULL,
    "kind"        TEXT                              NOT NULL,
    "recorded_at" TEXT                              NOT NULL
) STRICT;

CREATE INDEX "playback_history_play_id_idx" ON "playback_history_events" ("play_id");
CREATE INDEX "playback_history_recorded_at_idx" ON "playback_history_events" ("recorded_at");
