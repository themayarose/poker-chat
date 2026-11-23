$version = "2.7.0"

Get-ChildItem "$env:USERPROFILE\.nuget\packages\microsoft.windowsappsdk.winui\$version" -Recurse -Filter *.winmd | Copy-Item -Destination winmd
Get-ChildItem "$env:USERPROFILE\.nuget\packages\microsoft.windowsappsdk.interactiveexperiences\$version" -Recurse -Filter *.winmd | Copy-Item -Destination winmd
Get-ChildItem "$env:USERPROFILE\.nuget\packages\microsoft.windowsappsdk.foundation\$version" -Recurse -Filter *.winmd | Copy-Item -Destination winmd
