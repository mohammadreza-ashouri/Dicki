## Dicki

This is a simple Rust command-line tool to convert text files with a specific format into Anki dictionary files (CSV) for import into the Anki application.



## Build & Run
```
$ cargo build --release
$ ./target/release/Dicki input.txt output.csv
```

### How to use Dicki

The generated CSV file (output.csv) can be imported into Anki. To do that, follow these steps:

1-Open the Anki desktop application.

2- Click "File" > "Import".

3- Locate and select the generated CSV file (output.csv).

4- Choose the deck you want to import the cards into or create a new one.

5- Set the "Type" to "Basic".

6- Make sure the "Fields separated by" option is set to "Comma".

7- Map the fields correctly: "Front" to "Field 1" and "Back" to "Field 2".

8- Click "Import".

9- After importing the cards into Anki desktop, sync your Anki account to have the new cards available on your iPhone.





## Author

Dr. Mo Ashouri
- Email: ashourics@gmail.com

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
