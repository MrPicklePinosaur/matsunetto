<div align="center">

# 松ネット / matsunetto

simple device monitoring and metrics system

</div>

**matsunetto** is a simple device monitoring system. **mnet_client** is
installed on client devices and periodically pushes metrics like battery life
and ram usage to **mnet_server**. these metrics can then be polled for using
**mnet_cli**.

## SETTING UP FOR DEVELOPMENT

install git hooks
```
$ just devsetup
```

make a personal copy of the environment variables
```
$ cp .env-example .env
```

