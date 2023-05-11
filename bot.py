import json
import time
import requests

mainRequest = requests.Session()
mainhex = 76561197960265728

with open("config.json") as confile:
    config = json.load(confile)

def pullTradeOffers():
    while True:
        params = {
            "key": config["key"],
            "get_sent_offers": "false",
            "get_received_offers": "true",
            "active_only": "true"
        }
        print("* Trade Bot pulling Trade Offers *")
        try:
            gTradeOffersResponse = mainRequest.get("https://api.steampowered.com/IEconService/GetTradeOffers/v1/", params=params)
            gTradeOffersResponse.raise_for_status()
        except requests.exceptions.HTTPError as err:
            print(f"Warning: Trade bot is unable to receive Trade Offers: {err}")
            break
        if gTradeOffersResponse.status_code <= 200:
            rec = json.loads(gTradeOffersResponse.text)
            if 'trade_offers_received' in rec['response']:
                trade_offers_received = rec['response']['trade_offers_received']
                for trade_offer in trade_offers_received:
                    if (trade_offer['trade_offer_state']==2):
                        if not ('items_to_give' in trade_offer):# and ('is_our_offer' == False in trade_offer):
                            print("Trade bot found a Trade Offer")
                            print("*" * 30)
                            tradeofferid = trade_offer["tradeofferid"]
                            partnerid = trade_offer["accountid_other"] + mainhex
                            tradeAccept(tradeofferid, str(partnerid))
            else:
                print("Trade Offer list is empty.")
        time.sleep(10)

def tradeAccept(tradeid, partnerid):
    mainRequest.get("https://steamcommunity.com/")
    sessionid = mainRequest.cookies.get("sessionid")
    cookies = {
        "sessionid": sessionid,
        "steamLoginSecure": f"{config['webcookie']}",
    }
    data = {
        "sessionid": sessionid,
        "serverid": "1",
        "tradeofferid": tradeid,
        "partnerid": partnerid,
        "captcha": ""
    }
    headers = {
        "Referer": f"https://steamcommunity.com/tradeoffer/{tradeid}/",
    }
    response = mainRequest.post(f"https://steamcommunity.com/tradeoffer/{tradeid}/accept", headers=headers, cookies=cookies, data=data)
    if response.status_code >= 400:
        print(f"Trade #{tradeid} cannot be accepted")
    else:
        cres = json.loads(response.text)
        print(f"Trade Offer ID = {tradeid}")
        print(f"Partner ID = {partnerid}")
        print(f"Trade #{tradeid} accepted new Trade ID #{cres['tradeid']}")
        print("*" * 30)


print("Trade Bot Started.")    
pullTradeOffers()
