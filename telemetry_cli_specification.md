# Specification of MoonBlokz Telemetry CLI client

The purpose is this application is to read commands from the standard input (or from the command line) and transfer them to moonblokz-telemetry-hub

Architecture: It is a tokio based Rust command line application.

Command line parameters:
- api-key: The API key that must sent with all requestst as a header
- moonblokz-telemetry-hub url: The url of moonblokz-telemetry-hub
- command: optional (the command to send). If a command is present the app only send this command and not waits for new ones from the standard input

Working model:
- commands can be sent to the moonblokz-telemetry-hub using HTTPS POST requests
- All requests must contain the api key as a header.
- the app wait for a command from the standard input (processing it after enter)
- available commands:
    - - `set_update_interval` — Sets telemetry update frequency for a time window.  
    - command format: set_update_interval(node_id=21,start_time=2025-10-23T15:32+01,end_time=2025-10-23T15:42+01,active_period=30,inactive_period=300)
    - the uploaded command format will the following
    **Payload:**  
    ```json
    {
      "command":"set_update_interval",
      "parameters":{
        "node id":21, //(optional value. If not present the command will be sent to all nodes, by the telemetry hub
        "start_time": "YYYY-MM-DDTHH:MM:SSZ",
        "end_time": "YYYY-MM-DDTHH:MM:SSZ",
        "active_period": 30,
        "inactive_period": 300
      }
    }
    ```
  - `set_log_level` — Changes log verbosity on the node.  
    command format: set_log_level(node_id=21,log_level=TRACE)
    possible leveles `{ "level": "TRACE" | "DEBUG" | "INFO" | "WARN" | "ERROR" }`  

    - the uploaded command format will the following
    **Payload:**  
    ```json
    {
      "command":"set_log_level",
      "parameters":{
        "node id":21, //(optional value. If not present the command will be sent to all nodes, by the telemetry hub
        "log_level": "TRACE",
      }
    }
  - `set_filter` — Updates log filter.  
    command format: set_log_filter(node_id=21,log_filter="tm")
    - the uploaded command format will the following
    **Payload:**  
    ```json
    {
      "command":"set_log_filter",
      "parameters":{
        "node id":21, //(optional value. If not present the command will be sent to all nodes, by the telemetry hub
        "log_filter": "tm",
      }
    }

  - `run_command` — Executes a raw command on the node.  
    command format: command(node_id=21,command="/BS")
    - the uploaded command format will the following
    **Payload:**  
    ```json
    {
      "command":"command",
      "parameters":{
        "node id":21, //(optional value. If not present the command will be sent to all nodes, by the telemetry hub
        "command": "tm",
      }
    }
    
  - `update_node` — Initiates the **node** firmware update process.  
  command format: update_node(node_id=21)
- the uploaded command format will the following
    **Payload:**  
    ```json
    {
      "command":"update_node",
      "parameters":{
        "node id":21, //(optional value. If not present the command will be sent to all nodes, by the telemetry hub
      }
    }

- `update_probe` — Initiates the **probe** update process.  
  command format: update_probe(node_id=21)
- the uploaded command format will the following
    **Payload:**  
    ```json
    {
      "command":"update_probe",
      "parameters":{
        "node id":21, //(optional value. If not present the command will be sent to all nodes, by the telemetry hub
      }
    }

  - `reboot_probe` — Reboots the Probe's Raspberry Pi Zero 
    command format: reboot_probe(node_id=21)
- the uploaded command format will the following
    **Payload:**  
    ```json
    {
      "command":"reboot_probe",
      "parameters":{
        "node id":21, //(optional value. If not present the command will be sent to all nodes, by the telemetry hub
      }
    }
- The application check commands and report an error message in case of syntax errors.
- If the command is sintatically correct the CLI send it to the server
- If the response code is 200, It writes ok
- If the response code is 4xx, It writes command error
- If the response code is 5xx, It writes server error
- for other response code it writes unknown response

- After a command is processed new commands can be entered
- the cli can be closed with the following commands: quit,exit,bye

