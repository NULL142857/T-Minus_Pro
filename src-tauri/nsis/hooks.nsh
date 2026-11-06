; T-Minus installer hooks

!macro NSIS_HOOK_PREINSTALL
  ; Close a running instance first, otherwise the exe is locked and the
  ; installer fails with "cannot open file for writing".
  nsExec::Exec 'taskkill /F /T /IM "T-Minus.exe"'
  Pop $0
  nsExec::Exec 'taskkill /F /T /IM "t-minus.exe"'
  Pop $0
  Sleep 900

  ; If the file is still locked (for example the app was started elevated
  ; and cannot be killed from here), say so clearly instead of letting the
  ; copy step fail with a cryptic "cannot open file for writing" dialog.
  ${If} ${FileExists} "$INSTDIR\${MAINBINARYNAME}.exe"
    ClearErrors
    FileOpen $0 "$INSTDIR\${MAINBINARYNAME}.exe" a
    ${If} ${Errors}
      MessageBox MB_ICONSTOP|MB_OK "T-Minus is still running. Please quit it completely, then click Retry."
      Quit
    ${Else}
      FileClose $0
    ${EndIf}
  ${EndIf}
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  ; The uninstaller only removes files it installed. The tasks folder is
  ; created at runtime, so remove it explicitly when "delete app data"
  ; is checked, and keep it when the box is left unchecked.
  ${If} $DeleteAppDataCheckboxState = 1
    RmDir /r "$INSTDIR\tasks"
  ${EndIf}
!macroend
