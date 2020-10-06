package com.github.toncherami.mpd.web.common.serializers

import com.fasterxml.jackson.core.JsonGenerator
import com.fasterxml.jackson.databind.SerializerProvider
import com.fasterxml.jackson.databind.ser.std.StdSerializer
import java.time.Duration

class DurationSerializer(klass: Class<Duration>? = null) : StdSerializer<Duration>(klass) {

    override fun serialize(value: Duration, gen: JsonGenerator, provider: SerializerProvider) {
        gen.writeStartObject()

        gen.writeNumberField("hours", value.toHoursPart())
        gen.writeNumberField("minutes", value.toMinutesPart())
        gen.writeNumberField("seconds", value.toSecondsPart())
        gen.writeNumberField("secondsTotal", value.toSeconds())

        gen.writeEndObject()
    }

}
