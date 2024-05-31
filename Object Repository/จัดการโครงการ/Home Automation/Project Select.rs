<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Project Select</name>
   <tag></tag>
   <elementGuidId>79ef7291-87f2-4e11-87b1-742deaa4e34c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjI2NmJlMmZjLTc5ZDQtNGVmMC05YTEyLTJkZmZjOGQ0OWFhOSIsIm5hbWUiOiJBbXBvcm5zYWsiLCJmYW1pbHlOYW1lIjoiQW5na2F0YXZhbmljaCIsImVtYWlsIjoiYW1wb3Juc2FrLmFAY3JlYXRpdmVtZS5jby50aCIsInJvbGUiOnsiaWQiOiI2NzMwZTY5Ny0xMGIzLTQxODItYjI0ZC05MmE1MjczOGNiZDEiLCJuYW1lIjoiQ1BOIEJJTSJ9LCJwaG9uZSI6IjA5Njk2OTk5MTAiLCJpc1N1cGVyQWRtaW4iOmZhbHNlLCJpc0hhc1BpbiI6ZmFsc2UsImlzUGluRW5hYmxlZCI6ZmFsc2UsImlhdCI6MTcxNzEyMTI5OSwibmJmIjoxNzE3MTIxMjk5LCJleHAiOjE3MTcyMDc2OTksImlzcyI6Imh0dHBzOi8vcmVzaWRlbnRpYWxmYW1pbHkuY2VudHJhbHBhdHRhbmEuY28udGgiLCJzdWIiOiJjZW50cmFscGF0dGFuYS5jby50aCJ9.RrwIrxFcr9TKliwrp3HccWqy7X2iLivO2hfB-ZntOFw</value>
      <webElementGuid>d8eab5d1-27a2-48c9-a97f-37fd628ea35b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>2e08f6ee-bab1-49cb-893c-9c3e5a39342f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://45.150.131.146:7090/api-backoffice/project/select</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>70b66971-7baa-4907-bec0-f591030d213f</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
