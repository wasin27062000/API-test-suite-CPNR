<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ตรวจสอบการแก้ไขอุปกรณ์ไม่สำเร็จ เนื่องจากไม่กรอกข้อมูล</name>
   <tag></tag>
   <elementGuidId>0d3975dd-dfdf-4fb0-9214-31efa0e97bb5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: \&quot;${device}\&quot;,\n  \&quot;image\&quot;: \&quot;https://residentialfamily-dev.centralpattana.co.th/dev/assets/upload/home-automation/e1189e96-0f7c-4f0d-b27d-e172b233ff6c.jpg\&quot;,\n  \&quot;name\&quot;: {\n    \&quot;th\&quot;: \&quot;\&quot;,\n    \&quot;en\&quot;: \&quot;\&quot;\n  },\n  \&quot;type\&quot;: \&quot;deepLink\&quot;,\n  \&quot;orderedBy\&quot;: 3,\n  \&quot;active\&quot;: false,\n  \&quot;projects\&quot;: [\n    {\n      \&quot;id\&quot;: \&quot;0da685d4-6389-4eb5-8753-afdbd5f95e61\&quot;,\n      \&quot;name\&quot;: {\n        \&quot;th\&quot;: \&quot;เอสเซ็นท์ พาร์ควิลล์ เชียงใหม่\&quot;,\n        \&quot;en\&quot;: \&quot;ESCENT PARK VILLE CHIANGMAI\&quot;\n      }\n    }\n  ],\n  \&quot;urlIOS\&quot;: \&quot;\&quot;,\n  \&quot;urlAndroid\&quot;: \&quot;\&quot;,\n  \&quot;storeUrlIOS\&quot;: \&quot;\&quot;,\n  \&quot;storeUrlAndroid\&quot;: \&quot;\&quot;\n}&quot;,
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
      <webElementGuid>560101c1-3843-437c-a59a-0f1ecd95922b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjI2NmJlMmZjLTc5ZDQtNGVmMC05YTEyLTJkZmZjOGQ0OWFhOSIsIm5hbWUiOiJBbXBvcm5zYWsiLCJmYW1pbHlOYW1lIjoiQW5na2F0YXZhbmljaCIsImVtYWlsIjoiYW1wb3Juc2FrLmFAY3JlYXRpdmVtZS5jby50aCIsInJvbGUiOnsiaWQiOiI2NzMwZTY5Ny0xMGIzLTQxODItYjI0ZC05MmE1MjczOGNiZDEiLCJuYW1lIjoiQ1BOIEJJTSJ9LCJwaG9uZSI6IjA5Njk2OTk5MTAiLCJpc1N1cGVyQWRtaW4iOmZhbHNlLCJpc0hhc1BpbiI6ZmFsc2UsImlzUGluRW5hYmxlZCI6ZmFsc2UsImlhdCI6MTcxNzEyMTI5OSwibmJmIjoxNzE3MTIxMjk5LCJleHAiOjE3MTcyMDc2OTksImlzcyI6Imh0dHBzOi8vcmVzaWRlbnRpYWxmYW1pbHkuY2VudHJhbHBhdHRhbmEuY28udGgiLCJzdWIiOiJjZW50cmFscGF0dGFuYS5jby50aCJ9.RrwIrxFcr9TKliwrp3HccWqy7X2iLivO2hfB-ZntOFw</value>
      <webElementGuid>0431e18c-f987-4056-b404-170aab3a8006</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PATCH</restRequestMethod>
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
      <id>490c66bc-a5ba-4e0b-9f44-cf7bc387fcb9</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.device</defaultValue>
      <description></description>
      <id>c7b53901-4df5-4bea-bac7-5b0aee97d87b</id>
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
