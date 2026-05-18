using Microsoft.Web.WebView2.Core;
using Microsoft.Web.WebView2.WinForms;
using System;
using System.Drawing;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;
using System.Windows.Forms;

namespace 塔防精灵AI
{
    public partial class Form1 : Form
    {
        #region Win32 API
        [DllImport("user32.dll")]
        private static extern bool GetCursorPos(out POINT lpPoint);

        [DllImport("user32.dll")]
        private static extern IntPtr WindowFromPoint(POINT Point);

        [DllImport("user32.dll", CharSet = CharSet.Auto)]
        private static extern int GetWindowText(IntPtr hWnd, StringBuilder lpString, int nMaxCount);

        [DllImport("user32.dll", CharSet = CharSet.Auto)]
        private static extern int GetClassName(IntPtr hWnd, StringBuilder lpClassName, int nMaxCount);

        [StructLayout(LayoutKind.Sequential)]
        public struct POINT
        {
            public int X;
            public int Y;
        }
        #endregion

        // 保存原来的图标
        private Bitmap _crosshairBitmap; // 改用 Bitmap 保存准星
        public Form1()
        {
            InitializeComponent();
            _crosshairBitmap = new Bitmap(pictureBox1.Image);

            // 自动绑定事件
            pictureBox1.MouseDown += pictureBox1_MouseDown;
            pictureBox1.MouseMove += pictureBox1_MouseMove;
            pictureBox1.MouseUp += pictureBox1_MouseUp;
        }

        private void Form1_Load(object sender, EventArgs e)
        {

        }

        // 你的准星绘制代码（保持不变）
        private void DrawCrosshairInPictureBox()
        {
            using (Bitmap bmp = new Bitmap(16, 16))
            using (Graphics g = Graphics.FromImage(bmp))
            {
                g.Clear(Color.Transparent);
                g.SmoothingMode = System.Drawing.Drawing2D.SmoothingMode.AntiAlias;

                int cx = 8;
                int cy = 8;
                int r = 6;

                g.DrawEllipse(Pens.Black, cx - r, cy - r, r * 2, r * 2);
                g.DrawLine(Pens.Black, cx - r, cy, cx + r, cy);
                g.DrawLine(Pens.Black, cx, cy - r, cx, cy + r);

                pictureBox1.Image = (Bitmap)bmp.Clone();
            }

            pictureBox1.SizeMode = PictureBoxSizeMode.CenterImage;
            pictureBox1.BackColor = Color.Transparent;
        }

        private void toolStrip1_ItemClicked(object sender, ToolStripItemClickedEventArgs e) { }
        private void backgroundWorker1_DoWork(object sender, System.ComponentModel.DoWorkEventArgs e) { }

        private void comboBox1_SelectedIndexChanged(object sender, EventArgs e)
        {
            string name = comboBox1.Text;

            switch (name)
            {
                case "豆包":
                    webView21.Source = new Uri("https://www.doubao.com/chat/");
                    break;
                case "deepseek":
                    webView21.Source = new Uri("https://chat.deepseek.com/");
                    break;
                case "千问":
                    webView21.Source = new Uri("https://tongyi.aliyun.com/qianwen/");
                    break;
                case "文心一言":
                    webView21.Source = new Uri("https://yiyan.baidu.com/");
                    break;
                case "智谱清言":
                    webView21.Source = new Uri("https://chatglm.cn/");
                    break;
                case "讯飞星火":
                    webView21.Source = new Uri("https://xinghuo.xfyun.cn/");
                    break;
                case "Kimi":
                    webView21.Source = new Uri("https://kimi.moonshot.cn/");
                    break;
                case "腾讯混元":
                    webView21.Source = new Uri("https://hunyuan.tencent.com/");
                    break;
            }
        }

        private void button17_Click(object sender, EventArgs e)
        {
            try
            {
                // 1. 创建一个 32x32 的准星图标（和Spy++一模一样）
                using (Bitmap bmp = new Bitmap(32, 32))
                using (Graphics g = Graphics.FromImage(bmp))
                using (MemoryStream ms = new MemoryStream())
                {
                    g.Clear(Color.Transparent);
                    Pen pen = new Pen(Color.Black, 1);

                    // 画十字准星
                    g.DrawLine(pen, 16, 4, 16, 28);    // 竖线
                    g.DrawLine(pen, 4, 16, 28, 16);    // 横线
                    g.FillRectangle(Brushes.Black, 14, 14, 4, 4); // 中心方块

                    // 2. 保存为图标文件到【程序根目录】
                    string savePath = Path.Combine(Application.StartupPath, "准星图标.ico");
                    Icon icon = Icon.FromHandle(bmp.GetHicon());
                    icon.Save(ms);
                    File.WriteAllBytes(savePath, ms.ToArray());

                    // 3. 提示成功
                    // ✅ 提示成功
                    MessageBox.Show("准星图标已生成！\n路径：" + savePath, "成功", MessageBoxButtons.OK, MessageBoxIcon.Information);

                    // ✅ 关键：点确定后自动打开文件夹并选中文件
                    string argument = "/select, \"" + savePath + "\"";
                    System.Diagnostics.Process.Start("explorer.exe", argument);
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show("生成失败：" + ex.Message, "错误", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        private void pictureBox1_MouseDown(object sender, MouseEventArgs e)
        {
            if (e.Button == MouseButtons.Left)
            {
                // 隐藏图片框里的准星
                pictureBox1.Image = null;

                // ✅ 正确设置鼠标为你的准星（修复报错）
                this.Cursor = new Cursor(_crosshairBitmap.GetHicon());
            }
        }

        private void pictureBox1_MouseMove(object sender, MouseEventArgs e)
        {
            if (e.Button == MouseButtons.Left)
            {
                if (GetCursorPos(out POINT pt))
                {
                    IntPtr hWnd = WindowFromPoint(pt);
                    if (hWnd != IntPtr.Zero)
                    {
                        textBox1.Text = hWnd.ToString("X8");

                        StringBuilder title = new StringBuilder(256);
                        GetWindowText(hWnd, title, 256);
                        textBox2.Text = title.ToString();

                        StringBuilder cls = new StringBuilder(256);
                        GetClassName(hWnd, cls, 256);
                        textBox3.Text = cls.ToString();
                    }
                }
            }
        }

        private void pictureBox1_MouseUp(object sender, MouseEventArgs e)
        {
            if (e.Button == MouseButtons.Left)
            {
                // 1. 恢复鼠标
                this.Cursor = Cursors.Default;

                // 2. 准星自动回到图片框
                pictureBox1.Image = _crosshairBitmap;
            }
        }
    }
}