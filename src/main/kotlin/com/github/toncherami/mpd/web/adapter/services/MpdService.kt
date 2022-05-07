package com.github.toncherami.mpd.web.adapter.services

interface MpdService : MpdReadOnlyService, MpdWriteOnlyService {

    fun commandList(fn: MpdWriteOnlyService.() -> Unit)

}
