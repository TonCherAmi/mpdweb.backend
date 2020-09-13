package com.github.toncherami.mpd.web.configs

import com.github.toncherami.mpd.web.properties.MpdProperties
import org.springframework.boot.context.properties.EnableConfigurationProperties
import org.springframework.context.annotation.Configuration

@Configuration
@EnableConfigurationProperties(MpdProperties::class)
class PropertiesConfig
