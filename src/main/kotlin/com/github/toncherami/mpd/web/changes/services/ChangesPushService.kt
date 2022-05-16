package com.github.toncherami.mpd.web.changes.services

import com.github.toncherami.mpd.web.changes.enums.Change

interface ChangesPushService {

    fun push(changes: List<Change>)

}
