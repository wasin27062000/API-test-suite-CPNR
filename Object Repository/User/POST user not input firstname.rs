<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST user not input firstname</name>
   <tag></tag>
   <elementGuidId>af8f0966-eaf2-45a5-8fa3-a991f9183074</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: null,\n  \&quot;name\&quot;: null,\n  \&quot;familyName\&quot;: \&quot;testing\&quot;,\n  \&quot;ssoUserName\&quot;: \&quot;testing\&quot;,\n  \&quot;phone\&quot;: \&quot;0987654321\&quot;,\n  \&quot;email\&quot;: \&quot;testing12@gmail.com\&quot;,\n  \&quot;password\&quot;: null,\n  \&quot;confirmPassword\&quot;: null,\n  \&quot;position\&quot;: \&quot;testing\&quot;,\n  \&quot;department\&quot;: \&quot;testing\&quot;,\n  \&quot;active\&quot;: true,\n  \&quot;role\&quot;: {\n    \&quot;id\&quot;: \&quot;${id_roles}\&quot;,\n    \&quot;name\&quot;: \&quot;${name_roles}\&quot;,\n    \&quot;createdAt\&quot;: \&quot;2021-12-07T07:54:38.325Z\&quot;,\n    \&quot;updatedAt\&quot;: \&quot;2022-05-26T03:24:28.000Z\&quot;,\n    \&quot;deletedAt\&quot;: null\n  },\n  \&quot;projects\&quot;: null,\n  \&quot;isReceiveContact\&quot;: true,\n  \&quot;isReceiveRepair\&quot;: true,\n  \&quot;isReceiveReport\&quot;: true\n}&quot;,
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
      <webElementGuid>942749f7-70c9-45dc-8943-ede9b96c02e2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>da502921-c594-467f-a6a4-d0df59f279b4</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://45.150.131.146:7090/api-backoffice/admin/user</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>3d4d1a93-afee-4784-8ca3-0ba0e47eabee</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_roles</defaultValue>
      <description></description>
      <id>f172fdfa-a0e7-44d4-940e-933648707398</id>
      <masked>false</masked>
      <name>id_roles</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.name_roles</defaultValue>
      <description></description>
      <id>f7f5d697-0b80-4d53-9054-6370e10b5610</id>
      <masked>false</masked>
      <name>name_roles</name>
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
