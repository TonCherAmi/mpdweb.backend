package com.github.toncherami.mpd.web.database.data

data class DatabaseAudioFormat(val bitDepth: Int, val samplingRate: Int, val numberOfChannels: Int) {

    companion object {
        fun of(formatString: String): DatabaseAudioFormat {
            val parts = formatString.split(':')

            if (parts.size != 3) {
                throw IllegalArgumentException("Unable to parse format string '$formatString'")
            }

            return DatabaseAudioFormat(
                bitDepth = parts[1].toIntOrNull() ?: -1,
                samplingRate = parts[0].toIntOrNull() ?: -1,
                numberOfChannels = parts[2].toIntOrNull() ?: -1,
            )
        }
    }

}
