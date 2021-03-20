package com.github.toncherami.mpd.web.database.services.impl

import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.database.dto.DatabaseCount
import com.github.toncherami.mpd.web.database.dto.DatabaseItem
import com.github.toncherami.mpd.web.database.dto.enums.DatabaseItemType
import com.github.toncherami.mpd.web.database.services.DatabaseService
import org.springframework.stereotype.Service

@Service
class DatabaseServiceImpl(private val mpdService: MpdService) : DatabaseService {

    override fun update() {
        mpdService.update()
    }

    override fun get(uri: String): List<DatabaseItem> {
        return mpdService.lsinfo(uri)
            .map(DatabaseItem::of)
            .filterNot { it.type === DatabaseItemType.PLAYLIST }
    }

    override fun count(uri: String): DatabaseCount {
        return mpdService.count("base", uri)
            .let(DatabaseCount::of)
    }

}
