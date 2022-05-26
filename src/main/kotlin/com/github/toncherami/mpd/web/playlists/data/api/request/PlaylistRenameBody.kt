package com.github.toncherami.mpd.web.playlists.data.api.request

import javax.validation.constraints.NotBlank

data class PlaylistRenameBody(
    @NotBlank
    val to: String
)
