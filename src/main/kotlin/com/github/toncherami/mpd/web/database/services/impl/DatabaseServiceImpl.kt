package com.github.toncherami.mpd.web.database.services.impl

import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.database.dto.DatabaseItem
import com.github.toncherami.mpd.web.database.services.DatabaseService
import org.springframework.stereotype.Service

@Service
class DatabaseServiceImpl(private val mpdService: MpdService) : DatabaseService {

    override fun update() {
        mpdService.update()
    }

    override fun get(uri: String): List<DatabaseItem> {
        return mpdService.lsinfo(uri).map(DatabaseItem::of)
    }

}
