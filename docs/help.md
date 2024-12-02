# Command-Line Help for `oci-runtime-utils`

This document contains the help content for the `oci-runtime-utils` command-line program.

**Command Overview:**

* [`oci-runtime-utils`↴](#oci-runtime-utils)
* [`oci-runtime-utils true`↴](#oci-runtime-utils-true)
* [`oci-runtime-utils inspect`↴](#oci-runtime-utils-inspect)
* [`oci-runtime-utils inspect directory`↴](#oci-runtime-utils-inspect-directory)
* [`oci-runtime-utils inspect file`↴](#oci-runtime-utils-inspect-file)
* [`oci-runtime-utils inspect mountinfo`↴](#oci-runtime-utils-inspect-mountinfo)
* [`oci-runtime-utils inspect capabilities`↴](#oci-runtime-utils-inspect-capabilities)
* [`oci-runtime-utils run`↴](#oci-runtime-utils-run)
* [`oci-runtime-utils bundle`↴](#oci-runtime-utils-bundle)
* [`oci-runtime-utils spec`↴](#oci-runtime-utils-spec)
* [`oci-runtime-utils patch`↴](#oci-runtime-utils-patch)

## `oci-runtime-utils`

A simple command line tool contains some utilities for testing, benchmarking
and analysing behaviors of OCI runtime CLI implementations.

**Usage:** `oci-runtime-utils [OPTIONS] [COMMAND]`

For further information, please refer to the README file.

Report bugs to https://github.com/black-desk/oci-runtime-utils/issues/new

###### **Subcommands:**

* `true` — Return 0
* `inspect` — Inspect the system environment of current namespace
* `run` — Run a container with specified OCI runtime
* `bundle` — Create a minimal OCI bundle
* `spec` — Generated a minimal OCI runtime config file
* `patch` — Apply a json-patch to an OCI runtime config file, then print result to stdout

###### **Options:**

* `-v` — Set verbosity level



## `oci-runtime-utils true`

Return 0

**Usage:** `oci-runtime-utils true`



## `oci-runtime-utils inspect`

Inspect the system environment of current namespaces. This command has some
subcommands that only print specific information. Call without any subcommand
specified to print all information.

**Usage:** `oci-runtime-utils inspect [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `directory` — Inspect contents of a directory
* `file` — Inspect a text file
* `mountinfo` — Inspect /proc/self/mountinfo
* `capabilities` — Inspect capabilities

###### **Options:**

* `-f`, `--format <FORMAT>` — Output format

  Default value: `yaml`

  Possible values: `json`, `yaml`, `toml`




## `oci-runtime-utils inspect directory`

Inspect contents of a directory

**Usage:** `oci-runtime-utils inspect directory <PATH>`

###### **Arguments:**

* `<PATH>` — Directory to list



## `oci-runtime-utils inspect file`

Inspect a text file

**Usage:** `oci-runtime-utils inspect file <PATH>`

###### **Arguments:**

* `<PATH>` — File to cat



## `oci-runtime-utils inspect mountinfo`

Inspect /proc/self/mountinfo

**Usage:** `oci-runtime-utils inspect mountinfo`



## `oci-runtime-utils inspect capabilities`

Inspect capabilities

**Usage:** `oci-runtime-utils inspect capabilities`



## `oci-runtime-utils run`

Run a container with specified OCI runtime

**Usage:** `oci-runtime-utils run <RUNTIME> [CONTAINER_ID]`

###### **Arguments:**

* `<RUNTIME>` — Absolute file path to OCI runtime CLI program
* `<CONTAINER_ID>` — Optional container id. A random id will be generated if not provided



## `oci-runtime-utils bundle`

Generate a minimal OCI bundle directory, with an empty rootfs and a minimal
config.json file, which bind this program into the container's PATH, and set
process.args to `["oci-runtime-utils", "true"]`.

**Usage:** `oci-runtime-utils bundle [OPTIONS]`

###### **Options:**

* `-b`, `--bundle <BUNDLE>` — Path to the generated bundle

  Default value: `.`



## `oci-runtime-utils spec`

Generate a minimal OCI runtime config file, which bind this program into the
container's PATH, and set process.args to `["oci-runtime-utils", "inspect"]`.

**Usage:** `oci-runtime-utils spec [OPTIONS]`

###### **Options:**

* `-o`, `--output <OUTPUT>` — Generated file

  Default value: `config.json`



## `oci-runtime-utils patch`

Apply a json-patch to an OCI runtime config file read from input, then print
result to output file.

**Usage:** `oci-runtime-utils patch [OPTIONS] [COMMANDS]...`

###### **Arguments:**

* `<COMMANDS>` — Patch config.json to run this command in container

  Default values: `/usr/local/bin/oci-runtime-utils`, `true`

###### **Options:**

* `-i`, `--input <INPUT>` — Input file

  Default value: `config.json`
* `-o`, `--output <OUTPUT>` — Output file

  Default value: `/dev/stdout`
* `--with-utils` — Patch config.json to enable oci-runtime-utils in container

  Default value: `false`
* `--patch <PATCH>` — The json-patch file to apply



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

