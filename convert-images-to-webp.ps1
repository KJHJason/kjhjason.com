# Change the dir accordingly, could use Read-Host but getting issues with drive not existing
$dir = "E:\Codes\Github Projects\Blog\res\photos"
$quality = 80

# Convert all png and jpg images to webp without images in subdir
$files = Get-ChildItem -Path $dir | Where-Object { $_.Extension -eq ".png" -or $_.Extension -eq ".jpg" -or $_.Extension -eq ".jpeg" }

foreach ($file in $files) {
    # create a converted folder and the necessary folders if required
    [string]$newFolder = [string]$dir + "\converted\"
    mkdir $newFolder -ea 0

    [string]$replaced = $newFolder + $file.BaseName + ".webp"

    # Since the cwebp doesn't preserve the image orientation for jpg/jpeg despite 
    # using -metadata all and using exiftool to copy the metadata from the original 
    # image to the webp, decided to use ImageMagick instead.
    if ($file.Extension -eq ".jpg" -or $file.Extension -eq ".jpeg") {
        Write-Output "Converting $file to webp using ImageMagick to $replaced"
        magick $file.FullName -auto-orient -quality $quality $replaced # Using https://www.imagemagick.org/script/download.php#windows
    } else {
        cwebp -q $quality -mt $file.FullName -o $replaced # https://developers.google.com/speed/webp/docs/using
    }
}
