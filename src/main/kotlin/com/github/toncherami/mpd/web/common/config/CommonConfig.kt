package com.github.toncherami.mpd.web.common.config

import org.springframework.boot.context.properties.ConfigurationPropertiesScan
import org.springframework.context.annotation.Configuration
import org.springframework.integration.annotation.IntegrationComponentScan
import org.springframework.integration.config.EnableIntegration

@Configuration
@EnableIntegration
@IntegrationComponentScan
@ConfigurationPropertiesScan(basePackages = ["com.github.toncherami.mpd.web"])
class CommonConfig
