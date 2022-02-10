package com.github.toncherami.mpd.web.database.services.impl

import com.github.toncherami.mpd.web.adapter.dto.MpdDatabaseItem
import com.github.toncherami.mpd.web.adapter.dto.MpdRegexFileFilter
import com.github.toncherami.mpd.web.adapter.services.MpdService
import com.github.toncherami.mpd.web.database.dto.DatabaseCount
import com.github.toncherami.mpd.web.database.dto.DatabaseDirectory
import com.github.toncherami.mpd.web.database.dto.DatabaseItem
import com.github.toncherami.mpd.web.database.dto.enums.DatabaseItemType
import com.github.toncherami.mpd.web.database.services.DatabaseService
import org.springframework.stereotype.Service
import java.lang.IllegalArgumentException

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

    override fun search(term: String): List<DatabaseItem> {
        val files = mpdService.search(
            MpdRegexFileFilter(regex = term)
        )

        val regex = makeSearchTermRegex(term)

        return files.flatMap { mpdDatabaseItem ->
            val databaseItem = DatabaseItem.of(mpdDatabaseItem)

            getUriMatches(databaseItem, regex)
        }.distinctBy(DatabaseItem::uri).sortedWith { a, b ->
            when {
                a.type == DatabaseItemType.DIRECTORY && b.type == DatabaseItemType.FILE -> -1
                a.type == DatabaseItemType.FILE && b.type == DatabaseItemType.DIRECTORY -> 1
                else -> compareValues(a.uri, b.uri)
            }
        }
    }

    override fun cover(uri: String): ByteArray {
        return mpdService.albumart(uri)
    }

    private fun makeSearchTermRegex(term: String): Regex {
        return Regex(
            pattern = ".*$term.*",
            option = RegexOption.IGNORE_CASE,
        )
    }

    private fun getUriMatches(databaseItem: DatabaseItem, regex: Regex): List<DatabaseItem> {
        if (databaseItem.type != DatabaseItemType.FILE) {
            throw IllegalArgumentException("expected a file")
        }

        val pathSegments = databaseItem.uri.split(MpdDatabaseItem.PATH_SEPARATOR)
            .filter(String::isNotBlank)

        return pathSegments.mapIndexedNotNull { index, entry ->
            when {
                !entry.matches(regex) -> {
                    null
                }

                index == pathSegments.lastIndex -> {
                    databaseItem
                }

                else -> {
                    val uri = pathSegments.take(index.inc())
                        .joinToString(MpdDatabaseItem.PATH_SEPARATOR)

                    DatabaseDirectory(uri)
                }
            }
        }
    }


}
