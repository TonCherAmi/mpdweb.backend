package com.github.toncherami.mpd.web.adapter.deserializers

import com.github.toncherami.mpd.web.adapter.utils.MPD_ERROR_RESPONSE_STATUS
import com.github.toncherami.mpd.web.adapter.utils.MPD_OK_RESPONSE_STATUS
import org.springframework.core.serializer.Deserializer
import org.springframework.integration.ip.tcp.serializer.SoftEndOfStreamException
import java.io.BufferedReader
import java.io.IOException
import java.io.InputStream
import java.io.InputStreamReader

class MpdDeserializer : Deserializer<String> {

    override fun deserialize(inputStream: InputStream): String {
        val reader = BufferedReader(InputStreamReader(inputStream, "UTF-8"))

        var done = false

        val str = buildString {
            while (!done) {
                try {
                    val line: String = reader.readLine()
                        ?: throw SoftEndOfStreamException()

                    append("$line\n")

                    if (line == MPD_OK_RESPONSE_STATUS || line.startsWith(MPD_ERROR_RESPONSE_STATUS)) {
                        done = true
                    }
                } catch (exception: IOException) {
                    throw SoftEndOfStreamException()
                }
            }
        }

        return str.trim()
    }

}
