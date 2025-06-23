## Roadmap to Evolve the Framework Toward a complete Backend Framework

1. **Asynchronous HTTP Server**
    - ~~Implement a server based on `tokio`/`hyper`.~~
    - ~~Accept TCP connections, parse requests using existing types, and generate responses.~~

2. **Routing System**
    - ~~Introduce a router that maps paths and HTTP methods to controllers.~~
    - ~~Support parameterized routes and route groups (prefixes, middleware).~~

3. **Controllers and Dependency Injection**
    - ~~Define a trait or structure for controllers.~~
    - ~~Add a dependency injection container to create required instances (services, database access, etc.).~~

4. **Middleware (Request Pipeline)**
    - ~~Allow adding middleware executed before or after controllers (authentication, logging).~~
    - ~~Each middleware receives the request, can modify it, and then passes control to the next step.~~

5. **Session and Cookie Management**
    - Securely store session data (in-memory, database, or key-value backend).
    - Provide a clear API for reading/writing cookies and handling session persistence.

6. **Security**
    - Implement authentication mechanisms (Basic, tokens, sessions, OAuth).
    - Provide fine-grained authorization and protection against common attacks (CSRF, XSS).

7. **Database Access and ORM**
    - Integrate a Rust ORM (e.g., Diesel or SeaORM).
    - Offer an abstraction layer for migrations, entity definitions, and model relationships.

8. **Template Engine**
    - Choose a templating solution (e.g., Tera or Askama) to generate HTML views.
    - Manage template organization (layouts, inheritance) and provide helper functions.

9. **CLI Tools and Project Structure**
    - Create commands to generate a new project’s structure, ~~run the development server~~, and execute tests.
    - Document best practices and folder conventions (controllers, views, configurations).

10. **Testing, Documentation, and Continuous Integration**
    - Extend the test suite with functional scenarios (simulated requests).
    - Generate comprehensive documentation from code comments and maintain usage guides.
    - Set up CI to verify compilation and tests on each contribution.

11. **Advanced Features**
    - Forms (validation, CSRF) and HTML helpers.
    - Background task management (asynchronous jobs).
    - Optional support for WebSockets or real-time protocols.
