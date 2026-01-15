using System.Diagnostics;
using System.IO;

namespace MidnightSentinel
{
    public class MidnightSentinelTrayApp : Form
    {
        // Constants
        private const int CURSOR_OFFSET_DISTANCE = 100;
        private const string EMBEDDED_RESOURCE_PREFIX = "MidnightSentinel.Resources.";

        public NotifyIcon? trayIcon;
        public ContextMenuStrip? trayMenu;
        private readonly List<WpfOverlay> allOverlayInstances = [];
        private readonly Lock overlayLock = new();
        private bool isOverlayActive = false;
        private User32Interop.POINT activationCursorPosition;
        private readonly bool utilityMode = false;

        public MidnightSentinelTrayApp(bool runNow = false)
        {
            utilityMode = runNow;
            InitializeForm();
            InitializeTrayIcon();

            if (utilityMode)
            {
                // Hide tray icon in run-now mode
                if (trayIcon != null)
                    trayIcon.Visible = false;
                // Activate overlay immediately
                this.Load += (sender, e) => ActivateOverlay();
            }
        }

        private void InitializeForm()
        {
            this.WindowState = FormWindowState.Minimized;
            this.ShowInTaskbar = false;
        }

        private void InitializeTrayIcon()
        {
            trayMenu = new ContextMenuStrip();

            ApplyDarkTheme(trayMenu);

            Image? aboutIcon = LoadEmbeddedImage("about.png");
            var aboutMenuItem = new ToolStripMenuItem("About", aboutIcon, OnAbout)
            {
                ForeColor = Color.FromArgb(220, 220, 220), // Light gray
            };
            _ = trayMenu.Items.Add(aboutMenuItem);

            _ = trayMenu.Items.Add(new ToolStripSeparator());

            Image? exitIcon = LoadEmbeddedImage("exit.png");
            var exitMenuItem = new ToolStripMenuItem("Exit", exitIcon, OnExit)
            {
                ForeColor = Color.FromArgb(220, 220, 220), // Light gray
            };
            _ = trayMenu.Items.Add(exitMenuItem);

            Icon icon = LoadTrayIcon();

            trayIcon = new NotifyIcon()
            {
                Text = "Midnight Sentinel (" + WindowHelpers.GetVersionString() + ")",
                Icon = icon,
                ContextMenuStrip = trayMenu,
                Visible = true,
            };

            trayIcon.DoubleClick += (sender, args) => ActivateOverlay();
        }

        private static Icon LoadTrayIcon()
        {
            try
            {
                Image? image = LoadEmbeddedImage("systray.png");
                if (image != null)
                {
                    using var bitmap = new Bitmap(image);
                    return Icon.FromHandle(bitmap.GetHicon());
                }
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Error loading tray icon: {ex.Message}");
            }

            // Fallback to system icon
            return SystemIcons.Application;
        }

        private static void ApplyDarkTheme(ContextMenuStrip menu)
        {
            menu.Renderer = new ToolStripProfessionalRenderer(new DarkThemeColorTable());
        }

        private static Image? LoadEmbeddedImage(string imageName)
        {
            try
            {
                var assembly = System.Reflection.Assembly.GetExecutingAssembly();
                string resourceName = $"{EMBEDDED_RESOURCE_PREFIX}{imageName}";

                using var stream = assembly.GetManifestResourceStream(resourceName);
                if (stream != null)
                {
                    Debug.WriteLine($"Successfully loaded embedded resource: {resourceName}");
                    return Image.FromStream(stream);
                }
                else
                {
                    Debug.WriteLine($"Embedded resource not found: {resourceName}");

                    // Debug: List all available resources if the specific one isn't found
                    var resourceNames = assembly.GetManifestResourceNames();
                    Debug.WriteLine("Available embedded resources:");
                    foreach (var name in resourceNames)
                    {
                        Debug.WriteLine($"  - {name}");
                    }

                    return null;
                }
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Error loading embedded image '{imageName}': {ex.Message}");
                return null;
            }
        }

        public void ActivateOverlay()
        {
            lock (overlayLock)
            {
                if (isOverlayActive)
                {
                    Debug.WriteLine("Overlay creation attempt blocked: overlay is already active");
                    return;
                }

                isOverlayActive = true;

                try
                {
                    TerminateOverlay();
                    HideMouseCursor();
                    CreateOverlays();
                }
                catch (InvalidOperationException ex)
                {
                    Debug.WriteLine($"Invalid operation during overlay creation: {ex.Message}");
                    isOverlayActive = false;
                    RestoreMouseCursor();
                }
                catch (System.ComponentModel.Win32Exception ex)
                {
                    Debug.WriteLine($"Win32 error during overlay creation: {ex.Message}");
                    isOverlayActive = false;
                    RestoreMouseCursor();
                }
                catch (Exception ex)
                {
                    Debug.WriteLine($"Unexpected error creating overlay windows: {ex.Message}");
                    isOverlayActive = false;
                    RestoreMouseCursor();
                }
            }
        }

        private void TerminateOverlay()
        {
            foreach (var window in allOverlayInstances)
            {
                Debug.WriteLine($"Closing window on screen: {window.Screen.DeviceName}");
                window.Close();
            }
            allOverlayInstances.Clear();
        }

        private void CreateOverlays()
        {
            foreach (var screen in Screen.AllScreens)
            {
                Debug.WriteLine($"Creating WPF Overlay for screen: {screen.DeviceName}");

                var overlayWindow = new WpfOverlay(screen);

                overlayWindow.Show();
                overlayWindow.WindowState = System.Windows.WindowState.Normal;
                overlayWindow.Topmost = true;
                _ = overlayWindow.Activate();
                _ = overlayWindow.Focus();

                Debug.WriteLine(
                    $"Overlay created - Visible: {overlayWindow.IsVisible}, Active: {overlayWindow.IsActive}"
                );

                allOverlayInstances.Add(overlayWindow);
                overlayWindow.Closed += OnOverlayClosed;
            }
        }

        public void SetOverlayInactive()
        {
            lock (overlayLock)
            {
                isOverlayActive = false;
                Debug.WriteLine("Overlay active flag reset: overlay is no longer active.");

                if (utilityMode)
                {
                    Debug.WriteLine("Run-now mode: Exiting application after overlay dismissal.");
                    Application.Exit();
                }
            }
        }

        private void HideMouseCursor()
        {
            try
            {
                _ = User32Interop.GetCursorPos(out activationCursorPosition);
                _ = User32Interop.ShowCursor(false);
                MoveCursorOffScreen();
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Error hiding mouse cursor: {ex.Message}");
            }
        }

        public void RestoreMouseCursor()
        {
            try
            {
                _ = User32Interop.ShowCursor(true);
                RestoreCursorPosition();
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Error restoring mouse cursor: {ex.Message}");
            }
        }

        private void RestoreCursorPosition()
        {
            try
            {
                _ = User32Interop.GetCursorPos(out User32Interop.POINT deactivationCursorPosition);

                int deltaX = deactivationCursorPosition.X - activationCursorPosition.X;
                int deltaY = deactivationCursorPosition.Y - activationCursorPosition.Y;

                int newX = activationCursorPosition.X + deltaX;
                int newY = activationCursorPosition.Y + deltaY;

                // Validate cursor position is within screen bounds
                bool isValidPosition = false;
                foreach (var screen in Screen.AllScreens)
                {
                    if (screen.Bounds.Contains(newX, newY))
                    {
                        isValidPosition = true;
                        break;
                    }
                }

                if (isValidPosition)
                {
                    _ = User32Interop.SetCursorPos(newX, newY);
                    Debug.WriteLine(
                        $"Cursor restored: Original({activationCursorPosition.X},{activationCursorPosition.Y}) + Delta({deltaX},{deltaY}) = Final({newX},{newY})"
                    );
                }
                else
                {
                    // Fallback to original position if calculated position is invalid
                    _ = User32Interop.SetCursorPos(
                        activationCursorPosition.X,
                        activationCursorPosition.Y
                    );
                    Debug.WriteLine(
                        $"Cursor restored to original position due to invalid calculated position: ({activationCursorPosition.X},{activationCursorPosition.Y})"
                    );
                }
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Error restoring cursor position: {ex.Message}");
                // Attempt to restore to original position as fallback
                try
                {
                    _ = User32Interop.SetCursorPos(
                        activationCursorPosition.X,
                        activationCursorPosition.Y
                    );
                }
                catch (Exception fallbackEx)
                {
                    Debug.WriteLine(
                        $"Fallback cursor restoration also failed: {fallbackEx.Message}"
                    );
                }
            }
        }

        private static void MoveCursorOffScreen()
        {
            try
            {
                int maxRight = int.MinValue;
                int maxBottom = int.MinValue;

                foreach (var screen in Screen.AllScreens)
                {
                    maxRight = Math.Max(maxRight, screen.Bounds.Right);
                    maxBottom = Math.Max(maxBottom, screen.Bounds.Bottom);
                }

                // Ensure we have valid screen bounds
                if (maxRight > int.MinValue && maxBottom > int.MinValue)
                {
                    _ = User32Interop.SetCursorPos(
                        maxRight + CURSOR_OFFSET_DISTANCE,
                        maxBottom + CURSOR_OFFSET_DISTANCE
                    );
                }
                else
                {
                    Debug.WriteLine(
                        "Warning: Could not determine screen bounds for cursor positioning"
                    );
                }
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Error moving cursor off screen: {ex.Message}");
            }
        }

        private void OnAbout(object? sender, EventArgs e)
        {
            About.ShowAbout();
        }

        private void OnExit(object? sender, EventArgs e)
        {
            Application.Exit();
        }

        private void OnOverlayClosed(object? sender, EventArgs e)
        {
            lock (overlayLock)
            {
                if (sender is WpfOverlay closedWindow)
                {
                    _ = allOverlayInstances.Remove(closedWindow);
                    Debug.WriteLine(
                        $"Removed closed overlay for screen: {closedWindow.Screen.DeviceName}"
                    );
                }

                if (allOverlayInstances.Count == 0)
                {
                    isOverlayActive = false;
                    Debug.WriteLine("All overlay instances closed, overlay is no longer active");
                }
            }
        }

        protected override void Dispose(bool disposing)
        {
            if (disposing)
            {
                trayIcon?.Dispose();
                trayMenu?.Dispose();

                foreach (var overlay in allOverlayInstances)
                {
                    overlay.Close();
                }
            }

            base.Dispose(disposing);
        }
    }

    public class DarkThemeColorTable : ProfessionalColorTable
    {
        private static readonly Color _darkBackground = Color.FromArgb(32, 32, 32);
        private static readonly Color _selectedItem = Color.FromArgb(62, 62, 66);
        private static readonly Color _borderColor = Color.Gray;

        public override Color ImageMarginGradientBegin => _darkBackground;
        public override Color ImageMarginGradientMiddle => _darkBackground;
        public override Color ImageMarginGradientEnd => _darkBackground;
        public override Color ToolStripDropDownBackground => _darkBackground;
        public override Color MenuItemSelected => _selectedItem;
        public override Color MenuItemBorder => _borderColor;
        public override Color MenuBorder => _borderColor;
        public override Color MenuItemPressedGradientBegin => _selectedItem;
        public override Color MenuItemPressedGradientEnd => _selectedItem;
        public override Color MenuItemSelectedGradientBegin => _selectedItem;
        public override Color MenuItemSelectedGradientEnd => _selectedItem;
    }
}
