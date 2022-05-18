package com.github.toncherami.mpd.web.status.services

import com.github.toncherami.mpd.web.status.data.Status

interface StatusPushService {

    fun push(status: Status)

}
