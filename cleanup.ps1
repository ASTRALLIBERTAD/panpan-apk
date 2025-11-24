# PanPan Cleanup Script
# Removes old architecture files, keeping only the clean architecture

Write-Host "🧹 Cleaning up old PanPan architecture files..." -ForegroundColor Cyan
Write-Host ""

$filesDeleted = 0
$dirsDeleted = 0

# Function to safely remove items
function Remove-Safely {
    param($Path, $Description)
    if (Test-Path $Path) {
        Write-Host "  Removing $Description..." -ForegroundColor Yellow
        Remove-Item $Path -Recurse -Force -ErrorAction SilentlyContinue
        if (Test-Path $Path) {
            Write-Host "    ⚠️  Failed to remove: $Path" -ForegroundColor Red
        } else {
            if ((Get-Item $Path -ErrorAction SilentlyContinue) -is [System.IO.DirectoryInfo]) {
                $script:dirsDeleted++
            } else {
                $script:filesDeleted++
            }
            Write-Host "    ✓ Removed" -ForegroundColor Gray
        }
    }
}

# 1. Old build tools
Write-Host "[1/7] Old build tools..." -ForegroundColor Cyan
Remove-Safely "build_android.ps1" "build_android.ps1"
Remove-Safely "tools\panpan-apk" "tools\panpan-apk"
Remove-Safely "tools\panpan-desktop" "tools\panpan-desktop"

# 2. Old example crate
Write-Host "`n[2/7] Old example_crate..." -ForegroundColor Cyan
Remove-Safely "example_crate" "example_crate (replaced by demo_game)"

# 3. Old engine files
Write-Host "`n[3/7] Old engine files..." -ForegroundColor Cyan
Remove-Safely "panpan\src\engine.rs" "engine.rs"
Remove-Safely "panpan\src\platform.rs" "platform.rs"
Remove-Safely "panpan\src\platform" "platform directory"
Remove-Safely "panpan\src\renderer" "renderer directory"
Remove-Safely "panpan\src\util.rs" "util.rs"
Remove-Safely "panpan\build.rs" "panpan build.rs"

# 4. Old documentation
Write-Host "`n[4/7] Outdated documentation..." -ForegroundColor Cyan
Remove-Safely "CLEAN_ARCHITECTURE_STATUS.md" "old status doc"
Remove-Safely "ANDROID_BUILD.md" "old Android doc"
Remove-Safely "ANDROID_QUICK_START.md" "old quick start"

# 5. Build artifacts (optional - comment out if you want to keep)
Write-Host "`n[5/7] Build artifacts..." -ForegroundColor Cyan
Write-Host "  (These will be regenerated on next build)" -ForegroundColor Gray
Remove-Safely "target" "root target"
Remove-Safely "demo_runner\target" "demo_runner target"
Remove-Safely "examples\demo_game\target" "demo_game target"
Remove-Safely "runners\desktop\target" "desktop runner target"
Remove-Safely "runners\android\jni_wrapper\target" "jni_wrapper target"
Remove-Safely "panpan\target" "panpan target"

# 6. Android build outputs
Write-Host "`n[6/7] Android build outputs..." -ForegroundColor Cyan
Remove-Safely "runners\android\android\.gradle" ".gradle"
Remove-Safely "runners\android\android\app\build" "app build"
Remove-Safely "runners\android\android\build" "android build"
Remove-Safely "runners\android\jni_wrapper\android" "duplicate android folder"

# 7. Old crash log
Write-Host "`n[7/7] Old logs..." -ForegroundColor Cyan
Remove-Safely "crash.log" "old crash log"

# Summary
Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
Write-Host "✅ Cleanup Complete!" -ForegroundColor Green
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
Write-Host ""
Write-Host "Summary:" -ForegroundColor Cyan
Write-Host "  Files removed: $filesDeleted" -ForegroundColor White
Write-Host "  Directories removed: $dirsDeleted" -ForegroundColor White
Write-Host ""
Write-Host "Your project now contains only the clean architecture!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Test desktop: panpan run" -ForegroundColor White
Write-Host "  2. Test Android: panpan build --platform android" -ForegroundColor White
Write-Host ""
Write-Host "Documentation:" -ForegroundColor Yellow
Write-Host "  • README_FINAL.md - Complete guide"
Write-Host "  • CLI_GUIDE.md - Command reference"
Write-Host ""
