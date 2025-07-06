namespace MidnightSentinel
{
    internal class Program
    {
        [STAThread]
        public static void Main(string[] args)
        {
            bool runNow = args.Any(arg =>
                string.Equals(arg, "--run-now", StringComparison.OrdinalIgnoreCase)
            );

            if (runNow)
            {
                // In utility mode, bypass mutex check and run directly
                RunApplication(runNow);
            }
            else
            {
                // In tray app mode, check for existing instance
                bool createdNew;
                using Mutex mutex = new(true, "MidnightSentinelInstanceMutex", out createdNew);
                if (createdNew)
                {
                    RunApplication(runNow);
                }
                else
                {
                    // Initialize minimal WPF app for warning window
                    var wpfApp = new System.Windows.Application();
                    Warning.ShowWarning("Another instance is already running.");
                    wpfApp.Run();
                    return;
                }
            }
        }

        private static void RunApplication(bool runNow)
        {
            // Initialize WPF Application for overlays
            var wpfApp = new System.Windows.Application
            {
                ShutdownMode = System.Windows.ShutdownMode.OnExplicitShutdown,
            };

            // Initialize System Tray Application
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Application.Run(new MidnightSentinelTrayApp(runNow));

            wpfApp.Shutdown();
        }
    }
}
