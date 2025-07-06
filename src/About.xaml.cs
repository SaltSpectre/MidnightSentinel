using System;
using System.Diagnostics;
using System.Windows;
using System.Windows.Input;
using System.Windows.Media;
using WpfColor = System.Windows.Media.Color;

namespace MidnightSentinel
{
    public partial class About : Window
    {
        private static About? _instance;
        private readonly WpfColor _accentColor;

        public SolidColorBrush HoverColor { get; private set; }

        public About()
        {
            _accentColor = WindowHelpers.GetAccentColor();

            // Create hover color (lighter shade of accent)
            var hoverColor = WindowHelpers.LightenColor(_accentColor, 0.3f);
            HoverColor = new SolidColorBrush(hoverColor);

            DataContext = this;
            InitializeComponent();
            LoadContent();
        }

        public static void ShowAbout()
        {
            if (_instance != null)
            {
                _instance.Activate();
                _instance.Focus();
                return;
            }

            _instance = new About();
            _instance.Show();
        }

        private void LoadContent()
        {
            try
            {
                // Load logo from embedded resource
                var logoImage = WindowHelpers.LoadEmbeddedImage("MidnightSentinel.png");
                if (logoImage != null)
                {
                    LogoImage.Source = logoImage;
                }

                // Load name image
                var nameImage = WindowHelpers.LoadEmbeddedImage("Name_White.png");
                if (nameImage != null)
                {
                    NameImage.Source = nameImage;
                }

                // Set version
                VersionText.Text = WindowHelpers.GetVersionString();

                // Set accent color
                CloseButton.Background = new SolidColorBrush(_accentColor);
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Error loading content: {ex.Message}");
            }
        }

        private void OnGitHubLinkClick(object sender, MouseButtonEventArgs e)
        {
            try
            {
                Process.Start(
                    new ProcessStartInfo
                    {
                        FileName = "https://github.com/SaltSpectre/MidnightSentinel",
                        UseShellExecute = true,
                    }
                );
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Error opening GitHub link: {ex.Message}");
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
