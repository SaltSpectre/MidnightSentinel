using System;
using System.Diagnostics;
using System.Reflection;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using Microsoft.Win32;
using WpfColor = System.Windows.Media.Color;

namespace MidnightSentinel
{
    public static class WindowHelpers
    {
        public static BitmapImage? LoadEmbeddedImage(string imageName)
        {
            try
            {
                var assembly = Assembly.GetExecutingAssembly();
                string resourceName = $"MidnightSentinel.Resources.{imageName}";

                using var stream = assembly.GetManifestResourceStream(resourceName);
                if (stream != null)
                {
                    var bitmap = new BitmapImage();
                    bitmap.BeginInit();
                    bitmap.StreamSource = stream;
                    bitmap.CacheOption = BitmapCacheOption.OnLoad;
                    bitmap.EndInit();
                    bitmap.Freeze();
                    return bitmap;
                }
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Error loading embedded image '{imageName}': {ex.Message}");
            }

            return null;
        }

        public static string GetVersionString()
        {
            try
            {
                var assembly = Assembly.GetExecutingAssembly();
                var version = assembly.GetName().Version;
                if (version != null)
                {
                    return $"v{DateTime.Now:yyyy-MM-dd}";
                }
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Error getting version: {ex.Message}");
            }

            return $"v{DateTime.Now:yyyy-MM-dd}";
        }

        public static WpfColor GetAccentColor()
        {
            try
            {
                using var key = Registry.CurrentUser.OpenSubKey(@"SOFTWARE\Microsoft\Windows\DWM");
                var value = key?.GetValue("AccentColor");
                if (value is int colorValue)
                {
                    byte r = (byte)((colorValue >> 0) & 0xFF);
                    byte g = (byte)((colorValue >> 8) & 0xFF);
                    byte b = (byte)((colorValue >> 16) & 0xFF);
                    return WpfColor.FromRgb(r, g, b);
                }
            }
            catch
            {
                // Fallback to default blue
            }

            return WpfColor.FromRgb(0, 120, 212);
        }

        public static WpfColor LightenColor(WpfColor color, float amount)
        {
            float r = color.R / 255f;
            float g = color.G / 255f;
            float b = color.B / 255f;

            r = Math.Min(1f, r + amount);
            g = Math.Min(1f, g + amount);
            b = Math.Min(1f, b + amount);

            return WpfColor.FromRgb(
                (byte)(r * 255),
                (byte)(g * 255),
                (byte)(b * 255)
            );
        }
    }
}