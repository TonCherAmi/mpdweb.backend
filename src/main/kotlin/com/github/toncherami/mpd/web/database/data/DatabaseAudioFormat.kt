package com.github.toncherami.mpd.web.database.data

data class DatabaseAudioFormat(val bitDepth: Int, val samplingRate: Int, val numberOfChannels: Int) {

    companion object {
        fun of(formatString: String): DatabaseAudioFormat {
            val parts = formatString.split(':')

            if (parts.size != 3) {
                throw IllegalArgumentException("Unable to parse format string '$formatString'")
            }

            return DatabaseAudioFormat(
                bitDepth = parts[1].toInt(),
                samplingRate = parts[0].toInt(),
                numberOfChannels = parts[2].toInt()
            )
        }
    }

}
