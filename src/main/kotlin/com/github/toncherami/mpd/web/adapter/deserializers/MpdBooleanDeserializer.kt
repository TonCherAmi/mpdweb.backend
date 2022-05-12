package com.github.toncherami.mpd.web.adapter.deserializers

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.deser.std.StdDeserializer
import com.github.toncherami.mpd.web.adapter.deserializers.exceptions.MpdDeserializationException

class MpdBooleanDeserializer(klass: Class<Boolean>? = null) : StdDeserializer<Boolean>(klass) {

    override fun deserialize(p: JsonParser, ctxt: DeserializationContext): Boolean {
        return when (p.valueAsString) {
            "0" -> false
            "1" -> true
            else -> throw MpdDeserializationException(what = "MPD boolean", from = p.valueAsString)
        }
    }

}
