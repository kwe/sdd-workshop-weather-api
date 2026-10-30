From: Sarah Chen <sarah.chen@skycast.internal>
To: Platform Team <platform-team@skycast.internal>
CC: James Hargreaves <james.hargreaves@skycast.internal>
Date: 2026-06-23 09:14
Subject: RE: Forecast data — any plans?

Hi team,

Following up on the conversation from last week's all-hands. Three of us
(Dashboard team, Mobile team, and the Partner Integrations team) are all
hitting the same wall: we need forecast data and the Weather API doesn't
provide it.

Right now we're each doing our own thing:
- Dashboard team is scraping weatherapi.com and caching results locally
- Mobile team has a hardcoded 5-day placeholder that just repeats today's weather
- Partner Integrations team is just... not offering forecasts to partners at all

This is getting embarrassing. Partners are asking "why can't I get a forecast?"
and we're telling them "it's on the roadmap" — but it's been on the roadmap
for two quarters now.

What we need is simple: a forecast endpoint on the existing Weather API that
gives us day-by-day predictions for a city. We don't need hourly. We don't
need historical. Just: "what's the weather going to be like for the next 5 days
in London?"

Each day should have:
- High and low temperature
- What the weather will be like (sunny, rainy, etc.)
- Chance of rain (as a percentage)

The endpoint should work the same way as the current weather endpoints — same
URL patterns, same error format. We don't want to learn a new integration.

Can this be prioritised? Happy to join a refinement session if that helps.

Cheers,
Sarah

---

Sarah Chen
Senior Product Manager — Dashboard Team
SkyCast Ltd
