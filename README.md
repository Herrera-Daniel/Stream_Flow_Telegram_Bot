# Stream_Flow_Telegram_Bot

A simple telegram bot poorly written in rust that looks up streamflows in Colorado. In this first
version I had two main goals.

1. Learn a little rust.
2. Understand the apis for fetching up to date data about stream flows

My friends and I live in Colorado and love to fly fish. Often, when planning
trips in our telegram group, the subject of river flows comes up. This bot aims
to allow users to access the flows of their favorite rivers from their telegram
chats. In order to accomplish this the use of two apis were necessary,
the [CDSS REST service](https://dwr.state.co.us/Rest/GET/Help) and
the [Instantaneous Values Web Service](https://waterservices.usgs.gov/rest/)

# Supported Commands

/help - Displays available commands <br/>
/flow (stream name) - retrieves relevant flows for specified stream if
supported<br/>
/streams - Displays supported streams<br/>

# Supported Streams

Bear Creek (Morrison)<br/>
Big Thompson (Below Lake Estes)<br/>
Blue River (Above/Below Lake Dillon, Below Green Mountain Reservoir)<br/>
Clear Creek (Golden)<br/>
South Platte (Above Spinney, Dream Stream, Below 11 Mile Reservoir, Below
Cheeseman Reservoir, Deckers, North Fork Confluence, Below Strontia
Reservoir)<br/>
