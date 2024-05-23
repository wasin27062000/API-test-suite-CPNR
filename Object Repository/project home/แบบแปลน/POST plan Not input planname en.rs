<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST plan Not input planname en</name>
   <tag></tag>
   <elementGuidId>9dc0af78-0e6a-4eca-a91f-69bd9e3eb55f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;image\&quot;: null,\n  \&quot;name\&quot;: {\n    \&quot;th\&quot;: \&quot;แบบแปลนที่ 1\&quot;,\n    \&quot;en\&quot;: \&quot;Plan no.1\&quot;\n  },\n  \&quot;planName\&quot;: {\n    \&quot;th\&quot;: \&quot;แบบแปลนที่ 1\&quot;,\n    \&quot;en\&quot;: null\n  },\n  \&quot;items\&quot;: [\n    {\n      \&quot;image\&quot;: \&quot;https://residentialfamily-dev.centralpattana.co.th/dev/assets/upload/project/album/3b0548f7-1c12-4f6c-acdb-452861e38afc.jpg\&quot;,\n      \&quot;price\&quot;: 12,\n      \&quot;name\&quot;: {\n        \&quot;th\&quot;: \&quot;ตัวอย่างแบบแปลนที่ 1\&quot;,\n        \&quot;en\&quot;: \&quot;ex. plan no.1\&quot;\n      },\n      \&quot;planName\&quot;: {\n        \&quot;th\&quot;: \&quot;ตัวอย่างแบบแปลนที่ 1\&quot;,\n        \&quot;en\&quot;: \&quot;ex. plan no.1\&quot;\n      },\n      \&quot;landSize\&quot;: {\n        \&quot;th\&quot;: null,\n        \&quot;en\&quot;: null\n      },\n      \&quot;useableAreaSize\&quot;: {\n        \&quot;th\&quot;: \&quot;13\&quot;,\n        \&quot;en\&quot;: \&quot;12\&quot;\n      },\n      \&quot;bedRoom\&quot;: 2,\n      \&quot;toilet\&quot;: 4,\n      \&quot;carParking\&quot;: null,\n      \&quot;active\&quot;: null,\n      \&quot;orderBy\&quot;: 0\n    }\n  ],\n  \&quot;active\&quot;: null\n}&quot;,
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
      <webElementGuid>7b1bbb4a-8eea-4b4d-bebd-e3cd479ba690</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>6edc928d-d5b9-4266-b640-5b8ab7f03617</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://45.150.131.146:7090/api-backoffice/project/${id_project}/plan</restUrl>
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
      <id>aea45848-e614-4c37-aced-263ba69283f0</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_project</defaultValue>
      <description></description>
      <id>7382903f-d2f0-457a-8098-b64391ceb451</id>
      <masked>false</masked>
      <name>id_project</name>
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
