## Prerequisites

You'll need to install [docker](https://docs.docker.com/get-started/get-docker/) if you wish to run the service within a container.
If you want to run locally though you will need [rust](https://www.rust-lang.org/tools/install). Please ensure that you use the latest stable version, 1.80.0.

## Configuration

If you wish to change the address the API will be served on please edit the files within the configuration directory. When running from a built container image the application will use the host within production.yaml. When running from a binary built from this cloned repository the application will use the host within local.yaml. The port will be determined from the base.yaml file.
The host file is selected at runtime via the environment variable APP_ENVIRONMENT. The default is local but it can be changed by setting APP_ENVIRONMENT=production.

- Production `0.0.0.0:8080`
- Local `127.0.0.1:8080`

## Building

### Docker

Build the docker image

```
docker build --tag bis-api --file Dockerfile .
```

Run the built image

```
docker run -p 8080:8080 bis-api
```

### Local

Build and run with cargo

```
cargo run --bin bis-api
```

## API Documentation

After running bis-api, either through docker or locally you can access full API documentation by navigating to the running applications address within a browser and accessing the `/swagger-ui` endpoint.
The documentation located there is generated with [utoipa](https://github.com/juhaku/utoipa) and served as Swagger UI with [utoipa-swagger-ui](https://crates.io/crates/utoipa-swagger-ui).

# Book Information Service

## 1. Introduction

BIS or Book Information Service is a lightweight CRUD application that uses an in-memory data store to manage simple information about books, namely:

- Title
- Author
- Date_Published

The project has been made to showcase my personal knowledge and experience with, the Rust programming language, REST API design and development, Dockerization, and management of "External" dependencies. To represent my ability I have set out not to create a production ready solution but to fulfill the stated requirements and leave room for future development.

The following requirements were laid out by you:

1. Language: Rust
2. API Design: REST
3. Data Storage:
   1. in-memory store may be used
   2. Should be adaptable to support various database back-ends
4. Dockerization

### 1.2 Quality Goals

With requirements in mind certain aspects will decide on the overall quality of this project. I decided to focus largely on these:

- Maintainability
- Encapsulation
- Adaptability

These three quality goals ensure that this project could be adapted from a Book Information Store, with an in-memory storage solution. To any REST API focused on interacting with a Database.

## 2. Constraints

While there was few constraints to this project other than the outset requirements, one presents itself readily, time. While this project has been an enjoyable excuse to sit-down and truly code I only have so much time I can realistically spend on it. Given more time there are more features and functions that I would have liked to add, but every project has deadlines.

## 3. Solutions

### 3.1 Project Structure

In order to future proof this project I followed a structure that can also be seen within the [rust-analyzer](https://github.com/rust-lang/rust-analyzer) repository. I set the project up as a workspace with a virtual manifest that can track the settings I want replicated across each crate. Any new crate added to the `crates/` directory will be automatically included in the workspace members list, and if using `cargo` many of the workspace settings will be populated into their `cargo.toml`.
While this project structure is certainly a bit more complex than what was required for what I made, it also allows for growth and compartmentalizing. I represented this feature here by separating my in-memory store into it's own crate. By compartmentalizing the responsibilities of the API and storage into separate crates moving to a new storage solution will be far easier.

### 3.2 Data Storage -> (bis-in-memory)

Although I went with an in-memory store in order to meet my time constraints my goal was to match structure that one might find when working with [diesel](https://diesel.rs/) or [sqlx](https://github.com/launchbadge/sqlx). This is why when viewing the code you may see reference to `establish_connection()` or `pool` although there is no actual external connection or pool cf connections. The store its self is a Mutex wrapped hash-map indexing on a serial key. Any new book added to the store will be set to an id one larger than the current max. I am aware of the long-term limitations of a structure like this but it is only meant to be a short term tool.

While I did not make use of the technique here, for future expansion of this project an interface library made from traits and standard data types would be incredibly useful. It would allow us to define a standard wrapper around any data storage medium. We could then leverage those traits within the API crate and switch between any data store crate with those traits implemented.

### 3.3 Rest API

I stuck with simple CRUD operations within this project. I used [actix-web](https://actix.rs/) as my web framework. It is a feature rich library that sees a massive amount of use and support within the rust community. Beyond that, while it is incredibly extensible, you can also use it in a lightweight fashion.

Within `crates/bis-api/startup.rs` I create a web-server attached to a TcpListener and pass it the "connection" to the in-memory data store. This is also where I define the routes for the API endpoints, and API documentation. With actix routing you can use an Application scope to compose your applications. So if we wanted to add another CRUD service to this application we could have each API define their own service, import them to `startup.rs` and feed them into separate web::scope sections. You can also see here this is where I am serving the swagger-ui with the open api docs.

I have left `crates/bis-api/main.rs` intentionally light leaving only setting up configuration, a simple logger, our store connection, and a TcpListener. A main function should not have much "functionality" within it, it should focus on composing the various modules within a crate into a working whole.

`crates/bis-api/api.rs` makes up the bulk of the work within the bis-api crate. I only developed simple CRUD endpoints for this proof of concept but any more complex endpoints would be constructed in roughly the same way. The endpoints utilize actix-web extractors to pull path variables, query params, and pre-deserialized json bodies from requests as function params. They also access the in-memory store through our app-data. This is the same method you would likely use if your data store was instead a pool of db connections.
For error handling I have defined an api error type Book Error here and implemented the ResponseError trait. This gives the developer more ability to determine how a specific error is presented to the user through actix, handled by the system, and is interpreted by any kind of logging.
Important to any API as well is documentation. I chose a solution in which the documentation lives next to its corresponding endpoint, [utoipa](https://github.com/juhaku/utoipa). This crate offers a series of declarative macros to define OpenAPI docs. It also allows you to generate schema docs for request and response types as well as change the documented type and set format requirements.

### 3.4 Dockerization

The Dockerization requirement for this project is made relatively trivial because of the lack of any true external connection. If I was setting up this project with a DB I would likely use a docker compose file to launch this application and a PostgreSQL database together. I would also setup a volume for the Database to ensure data persistence between container rebuilds.
For my implementation of Dockerization I did use a good trick. Rust is a great candidate for multi-stage builds as it's binaries are statically linked. I split the build into two stages one builds the binary in a rust:latest container and the next runs it with correct configuration. The runtime container is also as small as I'm easily able to make it. This means that the built image is just over 100 Mb which makes it much easier to download.

## 4. Technical Debt

There are a few obvious aspects of the current implementation that will ultimately have to change before any progress can be made to a production ready solution.

1. Move away from an in-memory store. It is non-viable to have no form of data persistence. A single node faulting would wipe all work done on the system. Beyond that though it would be much harder to make this a horizontally scaling service without some form of central data store.
2. A wrapper crate with standard typing and traits would be required to make the transition from one data store to another. It would be relatively simple to transition this project from one database to another. That quality has quite a lot to do with this projects size. To make a much larger application have the same adaptability a robust wrapper crate and individual provider crates for each Database would be required.
3. Testing. I did not have the time required to write unit or integration tests along side my api code as I typically would.

## 5. Testing

I recommend using a Postman like tool to explore the api, I used [ApiDog](https://apidog.com/). Many of these tools, including ApiDog, will allow you to [import](https://apidog.com/help/importing-and-exporting-data/importing-data/importing-openapi-specification) via an open-api doc json file. You can find this open-api doc file within my repo under `api-doc.json`. After importing set up an environment with an address matching your application build method and you are ready to go.

## 6. Conclusion

This project has turned out to be a very enjoyable break from the job hunt so I want to thank you all for the opportunity. I believe I have created the start of a well structured, adaptable REST API solution. I'd love the chance to discuss the results of this exercise and your opinions on my design decisions.
