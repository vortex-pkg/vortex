- Run:
   * Seperate lifecycle scripts (start, restart, and test) from normal ones (should be relatively easy to do)
   * The env script is a special built-in command that can be used to list environment variables that will be available to the script at runtime. If an "env" command is defined in your package, it will take precedence over the built-in. (easy command)
   * In addition to the shell's pre-existing PATH, npm run adds node_modules/.bin to the PATH provided to scripts. Any binaries provided by locally-installed dependencies can be used without the node_modules/.bin prefix. For example, if there is a devDependency on tap in your package, you should write:
   * run sets the NODE environment variable to the node executable with which npm is executed.
- Init
  * The <initializer> argument in this case is an npm package named create-<initializer>, which will be installed by npm-exec, and then have its main bin executed -- presumably creating or updating package.json and running any other initialization-related operations.
