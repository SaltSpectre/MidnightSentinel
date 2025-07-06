using System;
using System.Diagnostics;
using System.Windows;
using System.Windows.Media;
using WpfColor = System.Windows.Media.Color;

namespace MidnightSentinel
{
    public partial class Warning : Window
    {
        private static Warning? _instance;
        private readonly WpfColor _accentColor;

        public SolidColorBrush HoverColor { get; private set; }

        public Warning(string message)
        {
            _accentColor = WindowHelpers.GetAccentColor();

            // Create hover color (lighter shade of accent)
            var hoverColor = WindowHelpers.LightenColor(_accentColor, 0.3f);
            HoverColor = new SolidColorBrush(hoverColor);

            DataContext = this;
            InitializeComponent();
            LoadContent(message);
        }

        public static void ShowWarning(string message)
        {
            if (_instance != null)
            {
                _instance.Activate();
                _instance.Focus();
                return;
            }

            _instance = new Warning(message);
            _instance.Show();
        }

        private void LoadContent(string message)
        {
            try
            {
                // Load warning icon from embedded resource
                var warningImage = WindowHelpers.LoadEmbeddedImage("warning.png");
                if (warningImage != null)
                {
                    WarningImage.Source = warningImage;
                }

                // Set warning message
                WarningText.Text = message;

                // Set accent color
                CloseButton.Background = new SolidColorBrush(_accentColor);
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Error loading warning content: {ex.Message}");
            }
        }

        private void OnCloseClick(object sender, RoutedEventArgs e)
        {
            Close();
        }

        protected override void OnClosed(EventArgs e)
        {
            _instance = null;
            base.OnClosed(e);
        }
    }
}
