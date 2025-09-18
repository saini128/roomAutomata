### Mobile App (Kotlin)
The mobile application is developed using **Kotlin** and serves as the user interface for controlling and monitoring the room's electrical appliances.

- **Technology Stack**:
    - **Language**: Kotlin
    - **UI Toolkit**: [Jetpack Compose](https://developer.android.com/jetpack/compose) for building the user interface.
    - **Networking**: [Retrofit](https://square.github.io/retrofit/) for handling communication with the Go server's REST API.
    - **Widgets**: [Jetpack Glance](https://developer.android.com/jetpack/compose/glance) for creating app widgets, allowing quick access to controls from the home screen.

- **User Interface**: The app features a single home page that displays all relevant information and controls. Each electrical appliance is represented by its own widget on this page for intuitive interaction.
- **Widget Functionality**: The app includes a home screen widget that provides quick access to essential controls and status information without needing to open the full application.
