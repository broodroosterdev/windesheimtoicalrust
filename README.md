# windesheimtoicalrust
## Intro
This repo contains a work in progress port of the windesheimtoical project to rust. The windesheimtoical project was built to convert a windesheim schedule into a ical file to allow users to view their schedule in their preferred calender program.


## Used Libraries


### Rocket
```js
rocket = "0.4.2"
```

Web framework used to create the get endpoints for the API. Responsible for all http communications to clients.

https://rocket.rs/ 

### Serde
```js
serde = "1.0"
serde_json = "1.0"
serde_derive = "1.0"
```

Framework for serializing and deserializing Rust data structures. Responsible for parsing the JSON data recieved by the windesheim api into the Les struct.

https://serde.rs/ 

### Reqwest
```js
reqwest = "0.9.22"
```

Highlevel HTTP client. Resposible for communications with windesheim's api.

https://github.com/seanmonstar/reqwest

### Chrono
```js
chrono = "0.4.9"
chrono-tz = "0.4"
```
Date and time library. Responsible for parsing the UNIX timestamps given by the windesheim api into iCal compliant datetime format.

https://github.com/chronotope/chrono 

### Ics
```js
ics = "0.4"
```
Library for creating iCalendar files as specified in RFC5545 and RFC7986. The real star of the show, as it is responsible for creating the actual ical files.

https://github.com/hummingly/ics 

