package com.github.toncherami.mpd.web.status.services

import com.github.toncherami.mpd.web.status.dto.Status

interface StatusPushService {

    fun push(status: Status)

}
