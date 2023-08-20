CREATE TABLE "playback_history_metadata" (
    "id"      INTEGER PRIMARY KEY,
    "play_id" INTEGER NOT NULL,
    "key"     TEXT    NOT NULL,
    "value"   TEXT    NULL
) STRICT;

CREATE INDEX "playback_history_metadata_play_id_idx" ON "playback_history_metadata" ("play_id");
