using System.Diagnostics;
using System.Windows;

namespace MidnightSentinel
{
    public class WpfOverlay : Window
    {
        public Screen Screen { get; private set; }

        public WpfOverlay(Screen screen)
        {
            Screen = screen;
            Debug.WriteLine($"Displaying WPF Overlay on screen: {screen.DeviceName}");

            InitializeWindow();
        }

        private void InitializeWindow()
        {
            this.WindowStyle = WindowStyle.None;
            this.ResizeMode = ResizeMode.NoResize;
            this.WindowState = WindowState.Normal;
            this.Topmost = true;
            this.Background = System.Windows.Media.Brushes.Black;
            this.ShowInTaskbar = false;
            this.AllowsTransparency = false;
            this.Visibility = Visibility.Visible;

            // Set window position and size to match screen bounds
            this.Left = Screen.Bounds.Left;
            this.Top = Screen.Bounds.Top;
            this.Width = Screen.Bounds.Width;
            this.Height = Screen.Bounds.Height;

            // Handle double-click to close overlays
            this.MouseDoubleClick += (sender, args) => CloseOverlays();
        }

        private static void CloseOverlays()
        {
            CloseAllOverlayWindows();
            ResetOverlayFlag();
        }

        private static void CloseAllOverlayWindows()
        {
            var windowsToClose = new List<Window>();
            foreach (Window window in System.Windows.Application.Current.Windows)
            {
                if (window is WpfOverlay)
                {
                    windowsToClose.Add(window);
                }
            }

            foreach (Window window in windowsToClose)
            {
                Debug.WriteLine(
                    $"Closing WPF Overlay on screen: {((WpfOverlay)window).Screen.DeviceName}"
                );
                window.Close();
            }
        }

        private static void ResetOverlayFlag()
        {
            MidnightSentinelTrayApp? trayApp = null;
            foreach (Form form in System.Windows.Forms.Application.OpenForms)
            {
                if (form is MidnightSentinelTrayApp app)
                {
                    trayApp = app;
                    break;
                }
            }

            if (trayApp != null)
            {
                trayApp.RestoreMouseCursor();
                trayApp.SetOverlayInactive();
            }
        }
    }
}
