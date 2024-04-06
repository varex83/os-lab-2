import argparse
import sys
import seaborn as sns
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

def save_combined_plots(file_path, file_data):
    data = []

    for line in file_data:
        parts = line.strip().split("::")
        if len(parts) != 2:
            print("Error: Each line must be in the format <file_name>:<file_size>")
            return
        try:
            size = int(parts[1])
        except ValueError:
            print("Error: File size must be an integer.")
            return
        data.append(size)

    # Generate plots
    with PdfPages(file_path) as pdf:
        plt.figure(figsize=(8, 6))
        plt.hist(data, bins=200, color='blue', alpha=0.9)
        plt.title('Histogram')
        plt.xlabel('Value')
        plt.ylabel('Frequency')
        pdf.savefig()
        plt.close()

        plt.figure(figsize=(8, 6))
        sns.kdeplot(data, color='green', fill=True)
        plt.title('Density Plot')
        plt.xlabel('Value')
        plt.ylabel('Density')
        pdf.savefig()
        plt.close()

        plt.figure(figsize=(8, 6))
        sns.violinplot(data, color='orange', inner='point')
        plt.title('Violin Plot')
        plt.xlabel('Value')
        plt.ylabel('Density')
        pdf.savefig()
        plt.close()

        plt.figure(figsize=(8, 6))
        plt.plot(data, 'bo', markersize=5)
        plt.title('Dot Diagram')
        plt.xlabel('Value')
        plt.ylabel('Density')
        pdf.savefig()
        plt.close()

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Combine plots into a single PDF file")
    parser.add_argument("-f", "--file", help="File path where to save the PDF file", default="combined_plots.pdf")
    args = parser.parse_args()

    # Read file names and sizes from stdin
    file_data = sys.stdin.readlines()

    save_combined_plots(args.file, file_data)

    print("".join(file_data))
