<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>กรณีลบอุปกรณ์สำเร็จ</name>
   <tag></tag>
   <elementGuidId>2ba3bd8e-b41d-4607-8606-11324128edb9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;:\&quot;${device}\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>dce287d1-671f-4fb7-b661-53dcac708038</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjI2NmJlMmZjLTc5ZDQtNGVmMC05YTEyLTJkZmZjOGQ0OWFhOSIsIm5hbWUiOiJBbXBvcm5zYWsiLCJmYW1pbHlOYW1lIjoiQW5na2F0YXZhbmljaCIsImVtYWlsIjoiYW1wb3Juc2FrLmFAY3JlYXRpdmVtZS5jby50aCIsInJvbGUiOnsiaWQiOiI2NzMwZTY5Ny0xMGIzLTQxODItYjI0ZC05MmE1MjczOGNiZDEiLCJuYW1lIjoiQ1BOIEJJTSJ9LCJwaG9uZSI6IjA5Njk2OTk5MTAiLCJpc1N1cGVyQWRtaW4iOmZhbHNlLCJpc0hhc1BpbiI6ZmFsc2UsImlzUGluRW5hYmxlZCI6ZmFsc2UsImlhdCI6MTcxNzEyMTI5OSwibmJmIjoxNzE3MTIxMjk5LCJleHAiOjE3MTcyMDc2OTksImlzcyI6Imh0dHBzOi8vcmVzaWRlbnRpYWxmYW1pbHkuY2VudHJhbHBhdHRhbmEuY28udGgiLCJzdWIiOiJjZW50cmFscGF0dGFuYS5jby50aCJ9.RrwIrxFcr9TKliwrp3HccWqy7X2iLivO2hfB-ZntOFw</value>
      <webElementGuid>009e4d5c-e5b5-45c5-9c02-a85ebc9af773</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>http://45.150.131.146:7090/api-backoffice/home-automation/deep-links</restUrl>
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
      <id>7275fd06-e4dc-4b2d-89f5-89f6e8216b4a</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.device</defaultValue>
      <description></description>
      <id>3fd51e9e-cc82-4d30-9d79-f6f8c95c61fc</id>
      <masked>false</masked>
      <name>device</name>
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
