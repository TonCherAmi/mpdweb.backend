package com.github.toncherami.mpd.web.changes.services.impl

import com.github.toncherami.mpd.web.adapter.data.MpdChange
import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.changes.enums.Change
import com.github.toncherami.mpd.web.changes.services.ChangesService
import org.springframework.stereotype.Service

@Service
class ChangesServiceImpl(
    private val mpdService: MpdService,
) : ChangesService {

    override fun get(): List<Change> {
        return mpdService.idle().map(MpdChange::changed)
    }

}
