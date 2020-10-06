package com.github.toncherami.mpd.web.changes.services

import com.github.toncherami.mpd.web.changes.enums.Change

interface ChangesService {

    fun get(): List<Change>
    fun send(changes: List<Change>)

}
