@echo off
REM --- CFG Engine build script (auto-loads VS tools) ---

if exist "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat" (
    call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1
) else if exist "C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvars64.bat" (
    call "C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1
) else if exist "C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Auxiliary\Build\vcvars64.bat" (
    call "C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1
) else (
    echo [ERROR] Could not find VS 2022 vcvars64.bat
    pause & exit /b 1
)
where ml64 >nul 2>&1 || ( echo [ERROR] ml64 not found & pause & exit /b 1 )
echo [+] VS build tools loaded

pushd output

echo [*] Building cfg_original...
ml64 /c /nologo cfg_original.asm
link cfg_original.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_original.exe
cfg_original.exe
echo     cfg_original exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_1...
ml64 /c /nologo cfg_variant_1.asm
link cfg_variant_1.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_1.exe
cfg_variant_1.exe
echo     cfg_variant_1 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_2...
ml64 /c /nologo cfg_variant_2.asm
link cfg_variant_2.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_2.exe
cfg_variant_2.exe
echo     cfg_variant_2 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_3...
ml64 /c /nologo cfg_variant_3.asm
link cfg_variant_3.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_3.exe
cfg_variant_3.exe
echo     cfg_variant_3 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_4...
ml64 /c /nologo cfg_variant_4.asm
link cfg_variant_4.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_4.exe
cfg_variant_4.exe
echo     cfg_variant_4 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_5...
ml64 /c /nologo cfg_variant_5.asm
link cfg_variant_5.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_5.exe
cfg_variant_5.exe
echo     cfg_variant_5 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_6...
ml64 /c /nologo cfg_variant_6.asm
link cfg_variant_6.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_6.exe
cfg_variant_6.exe
echo     cfg_variant_6 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_7...
ml64 /c /nologo cfg_variant_7.asm
link cfg_variant_7.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_7.exe
cfg_variant_7.exe
echo     cfg_variant_7 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_8...
ml64 /c /nologo cfg_variant_8.asm
link cfg_variant_8.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_8.exe
cfg_variant_8.exe
echo     cfg_variant_8 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_9...
ml64 /c /nologo cfg_variant_9.asm
link cfg_variant_9.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_9.exe
cfg_variant_9.exe
echo     cfg_variant_9 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_10...
ml64 /c /nologo cfg_variant_10.asm
link cfg_variant_10.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_10.exe
cfg_variant_10.exe
echo     cfg_variant_10 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_11...
ml64 /c /nologo cfg_variant_11.asm
link cfg_variant_11.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_11.exe
cfg_variant_11.exe
echo     cfg_variant_11 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_12...
ml64 /c /nologo cfg_variant_12.asm
link cfg_variant_12.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_12.exe
cfg_variant_12.exe
echo     cfg_variant_12 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_13...
ml64 /c /nologo cfg_variant_13.asm
link cfg_variant_13.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_13.exe
cfg_variant_13.exe
echo     cfg_variant_13 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_14...
ml64 /c /nologo cfg_variant_14.asm
link cfg_variant_14.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_14.exe
cfg_variant_14.exe
echo     cfg_variant_14 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_15...
ml64 /c /nologo cfg_variant_15.asm
link cfg_variant_15.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_15.exe
cfg_variant_15.exe
echo     cfg_variant_15 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_16...
ml64 /c /nologo cfg_variant_16.asm
link cfg_variant_16.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_16.exe
cfg_variant_16.exe
echo     cfg_variant_16 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_17...
ml64 /c /nologo cfg_variant_17.asm
link cfg_variant_17.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_17.exe
cfg_variant_17.exe
echo     cfg_variant_17 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_18...
ml64 /c /nologo cfg_variant_18.asm
link cfg_variant_18.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_18.exe
cfg_variant_18.exe
echo     cfg_variant_18 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_19...
ml64 /c /nologo cfg_variant_19.asm
link cfg_variant_19.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_19.exe
cfg_variant_19.exe
echo     cfg_variant_19 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_20...
ml64 /c /nologo cfg_variant_20.asm
link cfg_variant_20.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_20.exe
cfg_variant_20.exe
echo     cfg_variant_20 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_21...
ml64 /c /nologo cfg_variant_21.asm
link cfg_variant_21.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_21.exe
cfg_variant_21.exe
echo     cfg_variant_21 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_22...
ml64 /c /nologo cfg_variant_22.asm
link cfg_variant_22.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_22.exe
cfg_variant_22.exe
echo     cfg_variant_22 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_23...
ml64 /c /nologo cfg_variant_23.asm
link cfg_variant_23.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_23.exe
cfg_variant_23.exe
echo     cfg_variant_23 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_24...
ml64 /c /nologo cfg_variant_24.asm
link cfg_variant_24.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_24.exe
cfg_variant_24.exe
echo     cfg_variant_24 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_25...
ml64 /c /nologo cfg_variant_25.asm
link cfg_variant_25.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_25.exe
cfg_variant_25.exe
echo     cfg_variant_25 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_26...
ml64 /c /nologo cfg_variant_26.asm
link cfg_variant_26.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_26.exe
cfg_variant_26.exe
echo     cfg_variant_26 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_27...
ml64 /c /nologo cfg_variant_27.asm
link cfg_variant_27.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_27.exe
cfg_variant_27.exe
echo     cfg_variant_27 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_28...
ml64 /c /nologo cfg_variant_28.asm
link cfg_variant_28.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_28.exe
cfg_variant_28.exe
echo     cfg_variant_28 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_29...
ml64 /c /nologo cfg_variant_29.asm
link cfg_variant_29.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_29.exe
cfg_variant_29.exe
echo     cfg_variant_29 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_30...
ml64 /c /nologo cfg_variant_30.asm
link cfg_variant_30.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_30.exe
cfg_variant_30.exe
echo     cfg_variant_30 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_31...
ml64 /c /nologo cfg_variant_31.asm
link cfg_variant_31.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_31.exe
cfg_variant_31.exe
echo     cfg_variant_31 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_32...
ml64 /c /nologo cfg_variant_32.asm
link cfg_variant_32.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_32.exe
cfg_variant_32.exe
echo     cfg_variant_32 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_33...
ml64 /c /nologo cfg_variant_33.asm
link cfg_variant_33.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_33.exe
cfg_variant_33.exe
echo     cfg_variant_33 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_34...
ml64 /c /nologo cfg_variant_34.asm
link cfg_variant_34.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_34.exe
cfg_variant_34.exe
echo     cfg_variant_34 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_35...
ml64 /c /nologo cfg_variant_35.asm
link cfg_variant_35.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_35.exe
cfg_variant_35.exe
echo     cfg_variant_35 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_36...
ml64 /c /nologo cfg_variant_36.asm
link cfg_variant_36.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_36.exe
cfg_variant_36.exe
echo     cfg_variant_36 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_37...
ml64 /c /nologo cfg_variant_37.asm
link cfg_variant_37.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_37.exe
cfg_variant_37.exe
echo     cfg_variant_37 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_38...
ml64 /c /nologo cfg_variant_38.asm
link cfg_variant_38.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_38.exe
cfg_variant_38.exe
echo     cfg_variant_38 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_39...
ml64 /c /nologo cfg_variant_39.asm
link cfg_variant_39.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_39.exe
cfg_variant_39.exe
echo     cfg_variant_39 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_40...
ml64 /c /nologo cfg_variant_40.asm
link cfg_variant_40.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_40.exe
cfg_variant_40.exe
echo     cfg_variant_40 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_41...
ml64 /c /nologo cfg_variant_41.asm
link cfg_variant_41.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_41.exe
cfg_variant_41.exe
echo     cfg_variant_41 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_42...
ml64 /c /nologo cfg_variant_42.asm
link cfg_variant_42.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_42.exe
cfg_variant_42.exe
echo     cfg_variant_42 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_43...
ml64 /c /nologo cfg_variant_43.asm
link cfg_variant_43.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_43.exe
cfg_variant_43.exe
echo     cfg_variant_43 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_44...
ml64 /c /nologo cfg_variant_44.asm
link cfg_variant_44.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_44.exe
cfg_variant_44.exe
echo     cfg_variant_44 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_45...
ml64 /c /nologo cfg_variant_45.asm
link cfg_variant_45.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_45.exe
cfg_variant_45.exe
echo     cfg_variant_45 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_46...
ml64 /c /nologo cfg_variant_46.asm
link cfg_variant_46.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_46.exe
cfg_variant_46.exe
echo     cfg_variant_46 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_47...
ml64 /c /nologo cfg_variant_47.asm
link cfg_variant_47.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_47.exe
cfg_variant_47.exe
echo     cfg_variant_47 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_48...
ml64 /c /nologo cfg_variant_48.asm
link cfg_variant_48.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_48.exe
cfg_variant_48.exe
echo     cfg_variant_48 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_49...
ml64 /c /nologo cfg_variant_49.asm
link cfg_variant_49.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_49.exe
cfg_variant_49.exe
echo     cfg_variant_49 exit code: %ERRORLEVEL%

echo [*] Building cfg_variant_50...
ml64 /c /nologo cfg_variant_50.asm
link cfg_variant_50.obj /subsystem:console /entry:main kernel32.lib /nologo /out:cfg_variant_50.exe
cfg_variant_50.exe
echo     cfg_variant_50 exit code: %ERRORLEVEL%

popd
echo [+] Done. All exit codes should be 55 (fib(10)).
pause
