package com.github.toncherami.mpd.web.adapter.deserializers.data.enums

enum class MpdErrorCode(val value: Int) {

    UNK(-1),

    NOT_LIST(1),
    ARG(2),
    PASSWORD(3),
    PERMISSION(4),
    UNKNOWN_CMD(5),

    NO_EXIST(50),
    PLAYLIST_MAX(51),
    SYSTEM(52),
    PLAYLIST_LOAD(53),
    UPDATE_ALREADY(54),
    PLAYER_SYNC(55),
    EXIST(56),

}
