# Stock alerts


## Description

This script is meant to be run periodicaly.  
It will save the value of different stock in a csv file.  

Depending on a trigger csv file and the value of the script will trigger an alert.


It is meant to be run on AWS, with 
- script in rust
- lambda to launch the script
- S3 to store the CSV
- notification to send e-mail to user
- stock data from yahoo API


## configuration
- APi key for stock api
- destination e-mail
- list of stock to follow with key/pair of stock reference / human readeable name

## Steps

1. get the project csv with the different rules
2. From a list of stocks it will do the next steps
	1. download the current price of the action (yahoo API)
	2. download the project csv matching the current stock
	3. add the new price at the end of the csv
	4. save the modified csv
	5. check if any alert needs to be rised for this stock
3. gather all the triggers from different stock and send a e-mail


## stock csv file
name of the file: stock reference
- date
- value


## trigger CSV file
name: stock_alert_trigger.csv
- creation date
- stock reference

## mail template

### no trigger
```
subject: [stock alert] No alerts for this period
content:
There are no alert for the period.

<h3>Stock progression</h3>
name		price		last period progression		last 6 month
stock 1		25.4		7.3% [arrow up]			2.1% [arrow up]
stock 2		75.4		-5.6% [arrow down]		0.03% [arrow up]
stock 3		953.8		1.2% [arrow up]			-15% [arrow down]
stock 4		0.045		-0.5% [arrow down]		-0.2% [arrow down]
stock 5		12.34		8.32% [arrow up]		12.8% [arrow up]
```


### no trigger
```
subject: [stock alert] [triggered] 2 alerts for this period
content:
There is 2 alerts triggered for the period:
buy/sell	number	name		price	Comment
Buy		3	stock 2		75.4	Price really low	
sell		1	stock 4		0.045	

<h3>Stock progression</h3>
name		price		last period progression		last 6 month
stock 1		25.4		7.3% [arrow up]			2.1% [arrow up]
stock 2		75.4		-5.6% [arrow down]		0.03% [arrow up]
stock 3		953.8		1.2% [arrow up]			-15% [arrow down]
stock 4		0.045		-0.5% [arrow down]		-0.2% [arrow down]
stock 5		12.34		8.32% [arrow up]		12.8% [arrow up]

```

Buy and sell lines should be in green and red highlight.



