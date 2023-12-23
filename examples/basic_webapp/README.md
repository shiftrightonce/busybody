# Basic webapp

---
This example illustrates how a registered service can be use across threads in an actix web application.
The application is displaying the hours, minutes and seconds it has been online/up.

## Flow

- An instance of the ServiceUptime struct is created and registered in the service container
- The handlers for the following routes consume the service via different means
  - `/` : uptime -> uses the `service_container` function to get and use the `service_container`
  - `/two` : uptime2 -> uses the helper `service` to pluck the `ServiceUptime` instance directly
