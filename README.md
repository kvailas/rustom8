# rustom8
A rust native, automation engine.

### About rustom8

Rustom8 is a tiny binary program that can run workflow loads.
It specifically targets users interested in creating workflows 
that can run fast in the background.

### Features

Main activities include:

* Create workflow files using toml files.
* Define steps in your workflow file
* Each step type has specific options you can use to express your needs.
* Top-down workflow file execution.
* Auto shared context between steps. Reference data in your step from previous steps.
* Full-on multi-level debug logging.


## Step types

### Print
Prints a message to the console.

| Option      | Values |
|-------------|--------|
| **type**    | Print |
| **message** | A message string |


### Wait
Waits for a specified period of time in ms.

| Option          | Values             |
|-----------------|--------------------|
| **type**        | Wait               |
| **duration_ms** | A duration integer |


### Shell
Executes any command in the shell.

| Option      | Values           |
|-------------|------------------|
| **type**    | Shell            |
| **command** | A command string |


## Example workflow

**demo_workflow.toml**
```
name = "Demo Workflow 1"

[[steps]]
type = "Shell"
command = "echo 'Hello from rustom8'"

[[steps]]
type = "Wait"
duration_ms = 3000

[[steps]]
type = "Print"
message = "That's it! Waited for {{steps.3.output.Wait.duration}}"

[[steps]]
type = "Print"
message = "Previously, I printed: {{{steps.2.output.Print.message}}}. Hurray!!"


```

To run it:

```shell
rustom8 run demo_workflow.sh 
```